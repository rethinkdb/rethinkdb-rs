mod opt;

use crate::{
    cmd::connect::{Connection, RequestId},
    err, Result,
};
use bytes::{Buf, BufMut, Bytes, BytesMut, IntoBuf};
use futures::{channel::mpsc, prelude::*};
use romio::TcpStream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use self::opt::*;

const HEADER_LEN: usize = 8 + 4;

pub(crate) async fn run<O, T>(conn: &Connection, query: Bytes, _opts: O) -> Result<T>
where
    O: Into<Option<Opts>> + 'static,
    T: DeserializeOwned,
{
    if conn.broken() {
        return Err(err::Driver::ConnectionBroken)?;
    }
    let id = conn.token()?;
    let sess = Session::new(id, conn.stream());
    // TODO remove the hardcoded 10 after adding handling opts
    let len = query.len() + 10;
    let mut msg = BytesMut::with_capacity(len);
    msg.put("[1,");
    msg.put(query.as_ref());
    msg.put(",{}]");
    await!(sess.write(msg.as_ref()))?;
    match conn.controller() {
        Some(controller) => {
            let channel = mpsc::channel(conn.buffer());
            controller.insert(id, channel);
            Err(err::Driver::Other(String::from("unimplemented")))?
        }
        None => {
            let Response { resp, .. } = await!(conn.read())?;
            let mut msg: Message<_> = match serde_json::from_slice(&resp) {
                Ok(msg) => msg,
                Err(_) => {
                    return Err(err::Driver::UnexpectedResponse(resp.to_vec()))?;
                }
            };
            Ok(msg.r.pop().unwrap())
        }
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
pub(crate) struct Response<'a> {
    id: RequestId,
    resp: Bytes,
    stream: &'a TcpStream,
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
    pub(crate) async fn read<'a>(&'a self) -> Result<Response<'a>> {
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
        Ok(Response { id, resp, stream })
    }
}
