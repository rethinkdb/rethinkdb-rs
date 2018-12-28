mod opt;

use crate::{
    err, Result,
    cmd::connect::{RequestId, Connection},
};
use futures::{prelude::*, channel::mpsc};
use romio::TcpStream;
use bytes::{Bytes, BytesMut, BufMut};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub use self::opt::*;

const ID_LEN: usize = 8;
const DATA_LEN: usize = 4;
const HEADER_LEN: usize = ID_LEN + DATA_LEN;

pub(crate) async fn run<T, O>(conn: &Connection, query: Bytes, _opts: O) -> Result<T>
    where T: DeserializeOwned,
          O: Into<Option<Opts>> + 'static,
{
    if conn.broken() {
        return Err(err::Driver::ConnectionBroken)?;
    }
    let id = conn.token()?;
    let sess = Session::new(id, conn.stream());
    // TODO remove the hardcoded 10 after adding handling opts
    let len = query.len() + 10;
    let mut msg = BytesMut::with_capacity(len);
    msg.put(&b"[1,"[..]);
    msg.put(query.as_ref());
    msg.put(&b",{}]"[..]);
    await!(sess.write(msg.as_ref()))?;
    match conn.controller() {
        Some(controller) => {
            let channel = mpsc::channel(conn.buffer());
            controller.insert(id, channel);
            Err(err::Driver::Other(String::from("unimplemented")))?
        }
        None => {
            let Response { resp, .. } = await!(conn.read())?;
            let mut msg: Message<_> = serde_json::from_slice(&resp)?;
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
        await!(self.stream.write_all(&buf[..]))?;
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Response<'a> {
    id: RequestId,
    resp: Vec<u8>,
    stream: &'a TcpStream,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<T> {
    t: i32,
    e: Option<i32>,
    r: Vec<T>,
    b: Option<String>,
    p: Option<String>,
    n: Option<String>
}

impl Connection {
    pub(crate) async fn read<'a>(&'a self) -> Result<Response<'a>> {
        let mut header = [0u8; HEADER_LEN];
        let mut stream = self.stream();
        await!(stream.read_exact(&mut header))?;
        let mut token = [0u8; ID_LEN];
        token.copy_from_slice(&header[..ID_LEN]);
        let id = RequestId::from_le_bytes(token);
        let mut len_bytes = [0u8; DATA_LEN];
        len_bytes.copy_from_slice(&header[ID_LEN..]);
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut resp = vec![0; len];
        await!(stream.read_exact(&mut resp[..len]))?;
        Ok(Response { id, resp, stream })
    }
}
