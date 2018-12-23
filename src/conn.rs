use std::str;

use crate::{
    Client, Result, error,
    proto::Version,
};
use futures::prelude::*;
use romio::TcpStream;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Serialize, Deserialize};
use scram::client::{ScramClient, ServerFirst, ServerFinal};

const NULL_BYTE: u8 = b'\0';

#[derive(Debug)]
struct Session {
    id: u64,
    broken: bool,
    stream: TcpStream,
}

#[derive(Debug)]
pub struct Connection<'a> {
    client: Client<'a>,
    session: Session,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
    success: bool,
    min_protocol_version: usize,
    max_protocol_version: usize,
    server_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: i32,
    authentication_method: String,
    authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    success: bool,
    authentication: Option<String>,
    error_code: Option<usize>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
    authentication: String,
}

struct HandShake<'a> {
    // this should be enough for the handshake messages
    buf: [u8; 512],
    conn: Connection<'a>,
}

impl<'a> Connection<'a> {
    pub(crate) async fn new(client: Client<'a>, id: u64) -> Result<Connection<'a>> {
        let cfg = client.config();
        let stream = await!(TcpStream::connect(cfg.server()))?;
        let session = Session { id, stream, broken: false };
        let conn = Connection { client, session };
        let handshake = HandShake { conn, buf: [0u8; 512] };
        await!(handshake.greet())
    }
}

impl<'a> HandShake<'a> {
    async fn verify_version(&mut self) -> Result<&mut HandShake<'a>> {
        LittleEndian::write_u32(&mut self.buf, Version::V1_0 as u32);
        await!(self.conn.session.stream.write_all(&self.buf[..4]))?;
        await!(self.conn.session.stream.read(&mut self.buf))?;
        let resp = self.read_buf();
        match serde_json::from_slice::<ServerInfo>(resp) {
            Ok(info) => {
                if !info.success {
                    return Err(error::Runtime::Internal(resp.to_owned()))?;
                }
            }
            Err(_) => {
                let msg = str::from_utf8(resp)?;
                return Err(error::Driver::Other(msg.to_owned()))?;
            }
        }
        Ok(self)
    }

    async fn greet(mut self) -> Result<Connection<'a>> {
        await!(self.verify_version())?;

        // Send client first message
        let cfg = &self.conn.client.config();
        let scram = ScramClient::new(cfg.user(), cfg.password(), None)?;
        let (scram, msg) = client_first(scram)?;
        await!(self.conn.session.stream.write_all(&msg))?;

        // Send client final message
        await!(self.conn.session.stream.read(&mut self.buf))?;
        let info = AuthResponse::from_slice(self.read_buf())?;
        match info.authentication {
            Some(ref auth) => {
                let (scram, msg) = client_final(scram, auth)?;
                await!(self.conn.session.stream.write_all(&msg))?;
                await!(self.conn.session.stream.read(&mut self.buf))?;
                server_final(scram, self.read_buf())?;
                await!(self.conn.session.stream.flush())?;
            }
            None => {
                let msg = String::from("Server did not send authentication info.");
                return Err(error::Driver::Other(msg))?;
            }
        }

        Ok(self.conn)
    }

    fn read_buf(&self) -> &[u8] {
        let len = self.buf.iter()
            .take_while(|x| **x != NULL_BYTE)
            .count();
        &self.buf[..len]
    }
}

impl AuthResponse {
    fn from_slice(resp: &[u8]) -> Result<Self> {
        match serde_json::from_slice::<AuthResponse>(resp) {
            Ok(info) => {
                if !info.success {
                    // If error code is between 10 and 20, this is an auth error
                    if let Some(10...20) = info.error_code {
                        if let Some(msg) = info.error {
                            return Err(error::Driver::Auth(msg))?;
                        }
                    }
                    return Err(error::Runtime::Internal(resp.to_owned()))?;
                }
                Ok(info)
            }
            Err(_) => {
                let msg = str::from_utf8(resp)?;
                Err(error::Driver::Other(msg.to_owned()))?
            }
        }
    }
}

fn client_first<'a>(scram: ScramClient<'a>) -> Result<(ServerFirst<'a>, Vec<u8>)> {
    let (scram, client_first) = scram.client_first();
    let ar = AuthRequest {
        protocol_version: 0,
        authentication_method: String::from("SCRAM-SHA-256"),
        authentication: client_first,
    };
    let mut msg = serde_json::to_vec(&ar)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
}

fn client_final<'a>(scram: ServerFirst<'a>, auth: &str) -> Result<(ServerFinal, Vec<u8>)> {
    let scram = scram.handle_server_first(auth)?;
    let (scram, client_final) = scram.client_final();
    let conf = AuthConfirmation { authentication: client_final };
    let mut msg = serde_json::to_vec(&conf)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
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
