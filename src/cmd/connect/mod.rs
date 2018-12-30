mod opt;

use std::{
    net::SocketAddr,
    str,
    sync::atomic::{AtomicBool, AtomicU64, Ordering::SeqCst},
};

use crate::{err, r, Result};
use crossbeam_skiplist::SkipMap;
use futures::{channel::mpsc, prelude::*};
use romio::TcpStream;
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};

pub use self::opt::*;

const NULL_BYTE: u8 = b'\0';
const BUF_SIZE: usize = 512;
const PROTOCOL_VERSION: usize = 0;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    broken: AtomicBool,
    buffer: usize,
    multiplex: Option<Multiplex>,
}

enum Version {
    V1_0 = 0x34c2bdc3,
}

impl r {
    pub async fn connect<O>(self, opts: O) -> Result<Connection>
    where
        O: Into<Option<Opts>>,
    {
        let opt = opts.into().unwrap_or_default();
        let addr = SocketAddr::new(opt.host, opt.port);
        let stream = await!(TcpStream::connect(&addr))?;
        let multiplex = if opt.buffer != 0 {
            // Start counting from 1 because we want to use 0 to detect when the
            // token wraps over. If we allow tokens to be reused, the client may
            // return data not meant for that particular connection which may be
            // a security risk. This effectively means that a connection may be
            // used by `run` only up to `usize::max_value()` times.
            let counter = AtomicU64::new(1);
            let controller = SkipMap::new();
            Some(Multiplex {
                counter,
                controller,
            })
        } else {
            None
        };
        let conn = Connection {
            stream,
            multiplex,
            buffer: opt.buffer,
            broken: AtomicBool::new(false),
        };
        let handshake = HandShake {
            conn,
            buf: [0u8; BUF_SIZE],
        };
        await!(handshake.greet(opt))
    }
}

struct HandShake {
    // this should be enough for the handshake messages
    buf: [u8; BUF_SIZE],
    conn: Connection,
}

impl HandShake {
    // Performs the actual handshake
    //
    // This method optimises message exchange as suggested in the RethinkDB
    // documentation by sending message 3 right after message 1, without waiting
    // for message 2 first.
    async fn greet(mut self, opt: Opts) -> Result<Connection> {
        // Send the version we support
        let version = (Version::V1_0 as u32).to_le_bytes();
        await!(self.conn.stream.write_all(&version))?; // message 1

        // Send client first message
        let scram = ScramClient::new(&opt.user, &opt.password, None)?;
        let (scram, msg) = client_first(scram)?;
        await!(self.conn.stream.write_all(&msg))?; // message 3

        // Receive supported versions
        await!(self.conn.stream.read(&mut self.buf))?; // message 2
        let (len, info) = self.read_buf(0);
        ServerInfo::validate(info)?;

        // Receive server first message
        let offset = len + 1;
        let resp = if offset < BUF_SIZE && self.buf[offset] != NULL_BYTE {
            self.read_buf(offset).1
        } else {
            await!(self.conn.stream.read(&mut self.buf))?; // message 4
            self.read_buf(0).1
        };
        let info = AuthResponse::from_slice(resp)?;
        let auth = match info.authentication {
            Some(auth) => auth,
            None => {
                let msg = String::from("server did not send authentication info");
                return Err(err::Driver::Other(msg))?;
            }
        };

        // Send client final message
        let (scram, msg) = client_final(scram, &auth)?;
        await!(self.conn.stream.write_all(&msg))?; // message 5

        // Receive server final message
        await!(self.conn.stream.read(&mut self.buf))?; // message 6
        server_final(scram, self.read_buf(0).1)?;

        Ok(self.conn)
    }

    fn read_buf(&self, offset: usize) -> (usize, &[u8]) {
        let len = (&self.buf[offset..])
            .iter()
            .take_while(|x| **x != NULL_BYTE)
            .count();
        let max = offset + len;
        (max, &self.buf[offset..max])
    }
}

// We are going to use &str for `server_version` because it is safe to do so.
// Unfortunately, the other fields that are using String, are doing so because
// because they can potentially contain an escaped double quote which is not
// supported by serde in &str.
#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo<'a> {
    success: bool,
    min_protocol_version: usize,
    max_protocol_version: usize,
    server_version: &'a str,
}

impl<'a> ServerInfo<'a> {
    fn validate(resp: &[u8]) -> Result<()> {
        match serde_json::from_slice::<ServerInfo>(resp) {
            Ok(info) => {
                if !info.success {
                    return Err(err::Runtime::Internal(resp.to_owned()))?;
                }
                if info.min_protocol_version > PROTOCOL_VERSION
                    || PROTOCOL_VERSION > info.max_protocol_version
                {
                    let msg = format!(
                        "unsupported protocol version {version}, expected between {min} and {max}",
                        version = PROTOCOL_VERSION,
                        min = info.min_protocol_version,
                        max = info.max_protocol_version,
                    );
                    Err(err::Driver::Other(msg))?
                }
                Ok(())
            }
            Err(_) => {
                let msg = str::from_utf8(resp)?;
                Err(err::Driver::Other(msg.to_owned()))?
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: usize,
    authentication_method: &'static str,
    authentication: String,
}

fn client_first<'a>(scram: ScramClient<'a>) -> Result<(ServerFirst<'a>, Vec<u8>)> {
    let (scram, client_first) = scram.client_first();
    let ar = AuthRequest {
        protocol_version: PROTOCOL_VERSION,
        authentication_method: "SCRAM-SHA-256",
        authentication: client_first,
    };
    let mut msg = serde_json::to_vec(&ar)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
    authentication: String,
}

fn client_final<'a>(scram: ServerFirst<'a>, auth: &str) -> Result<(ServerFinal, Vec<u8>)> {
    let scram = scram.handle_server_first(auth)?;
    let (scram, client_final) = scram.client_final();
    let conf = AuthConfirmation {
        authentication: client_final,
    };
    let mut msg = serde_json::to_vec(&conf)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    success: bool,
    authentication: Option<String>,
    error_code: Option<usize>,
    error: Option<String>,
}

impl AuthResponse {
    fn from_slice(resp: &[u8]) -> Result<Self> {
        match serde_json::from_slice::<AuthResponse>(resp) {
            Ok(info) => {
                if !info.success {
                    // If error code is between 10 and 20, this is an auth error
                    if let Some(10...20) = info.error_code {
                        if let Some(msg) = info.error {
                            return Err(err::Driver::Auth(msg))?;
                        }
                    }
                    return Err(err::Runtime::Internal(resp.to_owned()))?;
                }
                Ok(info)
            }
            Err(_) => {
                let msg = str::from_utf8(resp)?;
                Err(err::Driver::Other(msg.to_owned()))?
            }
        }
    }
}

fn server_final(scram: ServerFinal, resp: &[u8]) -> Result<()> {
    let info = AuthResponse::from_slice(resp)?;
    if let Some(auth) = info.authentication {
        if let Err(err) = scram.handle_server_final(&auth) {
            return Err(err)?;
        }
    }
    Ok(())
}

impl Connection {
    pub(crate) fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub(crate) fn mark_broken(&self) {
        self.broken.store(true, SeqCst);
    }

    pub fn broken(&self) -> bool {
        self.broken.load(SeqCst)
    }

    pub(crate) fn buffer(&self) -> usize {
        self.buffer
    }

    pub(crate) fn token(&self) -> Result<RequestId> {
        match self.multiplex {
            Some(Multiplex { ref counter, .. }) => {
                let id = counter.fetch_add(1, SeqCst);
                // the token has wrapped over
                if id == 0 {
                    self.mark_broken();
                    return Err(err::Driver::TokenOverflow)?;
                }
                Ok(id)
            }
            None => Ok(1),
        }
    }

    pub(crate) fn controller(&self) -> Option<&Controller> {
        match self.multiplex {
            Some(Multiplex { ref controller, .. }) => Some(controller),
            None => None,
        }
    }
}

pub(crate) type RequestId = u64;
pub(crate) type Channel = (mpsc::Sender<()>, mpsc::Receiver<()>);
pub(crate) type Controller = SkipMap<RequestId, Channel>;

#[derive(Debug)]
pub(crate) struct Multiplex {
    counter: AtomicU64,
    controller: Controller,
}

#[cfg(test)]
mod tests {
    use crate::r;
    use futures::executor::block_on;

    #[test]
    fn driver_can_connect() -> crate::Result<()> {
        block_on(
            async {
                await!(r.connect(None))?;
                Ok(())
            },
        )
    }
}
