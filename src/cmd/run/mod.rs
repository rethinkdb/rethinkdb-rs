mod arg;
mod opt;
pub(crate) mod ser;

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
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use self::opt::*;

const HEADER_LEN: usize = 8 + 4;

macro_rules! runnable {
    ( $($cmd:ty,)* ) => {
        $(
            impl $cmd {
                pub fn run<'a, A, T>(self, arg: A) -> impl Future<Output=Result<T>> + 'a
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
    _opts: Option<Opts<'a>>,
) -> impl Future<Output = Result<T>> + 'a
where
    T: DeserializeOwned,
{
    async move {
        if conn.broken() {
            return Err(err::Driver::ConnectionBroken)?;
        }
        let id = conn.token()?;
        let sess = Session::new(id, conn.stream());
        let (header, footer) = ("[1,", ",{}]");
        let len = header.len() + query.len() + header.len();
        let mut msg = BytesMut::with_capacity(len);
        msg.put(header);
        msg.put(query);
        msg.put(footer);
        await!(sess.write(&msg))?;
        let Response { resp, .. } = await!(conn.read(id))?;
        let mut msg: Message<_> = match serde_json::from_slice(&resp) {
            Ok(msg) => msg,
            Err(_) => {
                return Err(err::Driver::UnexpectedResponse(resp.to_vec()))?;
            }
        };
        Ok(msg.r.pop().unwrap())
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
pub(crate) struct Response {
    id: RequestId,
    resp: Bytes,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<T> {
    t: i32,
    e: Option<i32>,
    r: Vec<T>,
    b: Option<Vec<String>>,
    p: Option<String>,
    n: Option<Vec<String>>,
}

impl Connection {
    async fn read_msg(&self) -> Result<Response> {
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
        Ok(Response { id, resp })
    }

    pub(crate) async fn read(&self, id: RequestId) -> Result<Response> {
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
