mod arg;
mod opt;
pub(crate) mod ser;

use std::ops::Deref;

use self::arg::Arg;
use crate::{
    cmd::{
        connect::{Connection, RequestId},
        *,
    },
    err, Result,
};
use bytes::{Buf, BufMut, Bytes, BytesMut, IntoBuf};
use futures::prelude::*;
use romio::TcpStream;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

pub use self::opt::*;

const HEADER_LEN: usize = 8 + 4;

macro_rules! runnable {
    ( $($cmd:ty,)* ) => {
        $(
            impl $cmd {
                pub fn run<'a, A, T>(self, arg: A) -> impl Future<Output=Result<Response<T>>> + 'a
                    where
                    A: Into<Arg<'a>>,
                    T: DeserializeOwned + 'static,
                    {
                        let Arg { conn, opts } = arg.into();
                        run(self.bytes, conn, opts)
                    }
            }
        )*
    }
}

runnable! {
    expr::Expr,
    merge::Merge,
    table::Table,
}

fn run<'a, T>(
    query: Bytes,
    conn: &'a Connection,
    mut opts: Opts<'a>,
) -> impl Future<Output = Result<Response<T>>> + 'a
where
    T: DeserializeOwned,
{
    async move {
        if conn.broken() {
            return Err(err::Driver::ConnectionBroken)?;
        }
        let id = conn.token()?;
        if opts.db.is_none() {
            let db = conn.db();
            if !db.is_empty() {
                opts.db(db);
            }
        }
        // We can't use `ser::to_vec` here because it will wrap the DB term in
        // an array. Luckily, the options to `run` do not contain arrays so we
        // can safely use the upstream `to_vec` here.
        let opts = serde_json::to_vec(&opts)?;
        let opts_len = opts.len();
        let (header, sep, footer) = ("[1,", ",", "]");
        let len = header.len() + query.len() + sep.len() + opts_len + footer.len();
        let mut msg = BytesMut::with_capacity(len);
        msg.put(header);
        msg.put(query);
        // don't include an empty object
        if opts_len > 2 {
            msg.put(sep);
            msg.put(opts);
        }
        msg.put(footer);
        log::debug!("query => {}", std::str::from_utf8(&msg).unwrap());
        let sess = Session::new(id, conn.stream());
        await!(sess.write(&msg))?;
        let Payload { resp, .. } = await!(conn.read(id))?;
        log::debug!("response => {}", std::str::from_utf8(&resp).unwrap());
        let msg: Message<_> = match serde_json::from_slice(&resp) {
            Ok(msg) => msg,
            Err(_) => {
                let msg: Message<Value> = serde_json::from_slice(&resp)?;
                return Err(err::Driver::UnexpectedResponse(msg.r))?;
            }
        };
        Ok(Response {
            value: msg.r,
            profile: msg.p.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Session<'a> {
    id: RequestId,
    stream: &'a TcpStream,
}

impl<'a> Session<'a> {
    pub(crate) fn new(id: RequestId, stream: &'a TcpStream) -> Self {
        Session { id, stream }
    }

    pub(crate) async fn write(mut self, data: &'a [u8]) -> Result<Session<'a>> {
        let data_len = data.len();
        let mut buf = BytesMut::with_capacity(HEADER_LEN + data_len);
        buf.put_u64_le(self.id);
        buf.put_u32_le(data_len as u32);
        buf.put(data);
        await!(self.stream.write_all(&buf))?;
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Payload {
    id: RequestId,
    resp: Bytes,
}

#[derive(Deserialize, Debug)]
struct Message<T> {
    t: u8,
    e: Option<u32>,
    r: Vec<T>,
    b: Option<Vec<Value>>,
    p: Option<Vec<Profile>>,
    n: Option<Vec<Value>>,
}

impl Connection {
    async fn read_msg(&self) -> Result<Payload> {
        let mut buf = BytesMut::new();
        buf.resize(HEADER_LEN, 0);
        let mut stream = self.stream();
        await!(stream.read_exact(&mut buf))?;
        let mut header = buf.take().into_buf();
        let id = header.get_u64_le();
        let len = header.get_u32_le() as usize;
        buf.resize(len, 0);
        await!(stream.read_exact(&mut buf))?;
        let resp = buf.freeze();
        Ok(Payload { id, resp })
    }

    pub(crate) async fn read(&self, id: RequestId) -> Result<Payload> {
        let mut resp = await!(self.read_msg())?;
        if resp.id != id {
            if let Some(controller) = self.controller() {
                let channel = crossbeam_channel::unbounded();
                controller.insert(id, channel);
                if let Some(entry) = controller.get(&id) {
                    let (ref tx, ref rx) = entry.value();
                    while resp.id != id {
                        // unwrapping here should be safe because we have just
                        // retrieved this channel and it's still in scope
                        tx.send(resp).unwrap();
                        resp = match rx.try_recv() {
                            Ok(resp) => resp,
                            Err(_) => await!(self.read_msg())?,
                        };
                    }
                }
            }
        }
        Ok(resp)
    }
}

#[derive(Debug, Clone)]
pub struct Response<T> {
    value: Vec<T>,
    profile: Vec<Profile>,
}

impl<T> Deref for Response<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    description: Option<String>,
    #[serde(rename = "duration(ms)")]
    duration: Option<f64>,
    sub_tasks: Option<Vec<Profile>>,
    parallel_tasks: Option<Vec<Vec<Profile>>>,
}
