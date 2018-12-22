use crate::{
    Client, Result, error,
    proto::Version,
};
use futures::prelude::*;
use romio::TcpStream;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Serialize, Deserialize};
use scram::client::ScramClient;

const NULL_BYTE: u8 = b'\0';

#[derive(Debug)]
struct Session {
    id: u64,
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

impl<'a> Connection<'a> {
    pub(crate) async fn new(client: Client<'a>, id: u64) -> Result<Connection<'a>> {
        let cfg = client.config();
        let stream = await!(TcpStream::connect(cfg.server()))?;
        let session = Session { id, stream };
        let conn = Connection { client, session };
        await!(conn.shake_hands())
    }

    async fn shake_hands(mut self) -> Result<Connection<'a>> {
        // verify version support on server
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, Version::V1_0 as u32);
        await!(self.session.stream.write_all(&buf))?;
        let mut resp = vec![];
        await!(self.session.read_until(NULL_BYTE, &mut resp))?;
        resp.pop();
        let info: ServerInfo = serde_json::from_slice(&resp)?;
        if !info.success {
            return Err(error::Runtime::Internal(resp))?;
        }

        // Send client first message
        let cfg = &self.client.config();
        let scram = ScramClient::new(cfg.user(), cfg.password(), None)?;
        let (scram, client_first) = scram.client_first();
        let ar = AuthRequest {
            protocol_version: 0,
            authentication_method: String::from("SCRAM-SHA-256"),
            authentication: client_first,
        };
        let mut msg = serde_json::to_vec(&ar)?;
        msg.push(NULL_BYTE);
        await!(self.session.stream.write_all(&msg))?;

        // Send client final message
        let mut resp = vec![];
        await!(self.session.read_until(NULL_BYTE, &mut resp))?;
        resp.pop();
        let info: AuthResponse = serde_json::from_slice(&resp)?;
        if !info.success {
            // If error code is between 10 and 20, this is an auth error
            if let Some(10...20) = info.error_code {
                // @TODO consider returning `AuthResponse` in the Auth error
                return Err(error::Driver::Auth(resp))?;
            }
            return Err(error::Runtime::Internal(resp))?;
        };
        match info.authentication {
            Some(auth) => {
                let scram = scram.handle_server_first(&auth)?;
                let (scram, client_final) = scram.client_final();
                let auth = AuthConfirmation { authentication: client_final };
                let mut msg = serde_json::to_vec(&auth)?;
                msg.push(NULL_BYTE);
                await!(self.session.stream.write_all(&msg))?;

                // Validate final server response and flush the buffer
                let mut resp = vec![];
                await!(self.session.read_until(NULL_BYTE, &mut resp))?;
                resp.pop();
                let info: AuthResponse = serde_json::from_slice(&resp)?;
                if !info.success {
                    // If error code is between 10 and 20, this is an auth error
                    if let Some(10...20) = info.error_code {
                        // @TODO consider returning `AuthResponse` in the Auth error
                        return Err(error::Driver::Auth(resp))?;
                    }
                    return Err(error::Runtime::Internal(resp))?;
                };
                if let Some(auth) = info.authentication {
                    if let Err(err) = scram.handle_server_final(&auth) {
                        return Err(err)?;
                    }
                }
                await!(self.session.stream.flush())?;
            }
            None => {
                let msg = String::from("Server did not send authentication info.");
                return Err(error::Driver::Other(msg))?;
            }
        }

        Ok(self)
    }
}

impl Session {
    // remove this once https://github.com/rust-lang-nursery/futures-rs/issues/1373 is resolved
    async fn read_until<'a>(&'a mut self, byte: u8, buf: &'a mut Vec<u8>) -> Result<()> {
	// this should be more than enough for the handshake responses
        let mut resp = [0u8; 1024];
        await!(self.stream.read(&mut resp))?;
        for b in resp.iter() {
            buf.push(*b);
            if *b == byte {
                break;
            }
        }
        Ok(())
    }
}
