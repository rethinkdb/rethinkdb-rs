use std::str;

use crate::{cmd::connect::Opts, err, net::connection::Connection, Result};
use futures::prelude::*;
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};

const NULL_BYTE: u8 = b'\0';
const BUF_SIZE: usize = 512;
const PROTOCOL_VERSION: usize = 0;

enum Version {
    V1_0 = 0x34c2_bdc3,
}

pub(crate) struct HandShake {
    // this should be enough for the handshake messages
    buf: [u8; BUF_SIZE],
    conn: Connection,
}

impl HandShake {
    pub(crate) fn new(conn: Connection) -> Self {
        Self {
            conn,
            buf: [0; BUF_SIZE],
        }
    }

    // Performs the actual handshake
    //
    // This method optimises message exchange as suggested in the RethinkDB
    // documentation by sending message 3 right after message 1, without waiting
    // for message 2 first.
    pub(crate) async fn greet<'a>(mut self, opt: Opts<'a>) -> Result<Connection> {
        let mut stream = self.conn.stream();

        // Send the version we support
        let version = (Version::V1_0 as u32).to_le_bytes();
        await!(stream.write_all(&version))?; // message 1

        // Send client first message
        let scram = ScramClient::new(opt.user, opt.password, None)?;
        let (scram, msg) = client_first(scram)?;
        await!(stream.write_all(&msg))?; // message 3

        // Receive supported versions
        await!(stream.read(&mut self.buf))?; // message 2
        let (len, info) = self.read_buf(0);
        ServerInfo::validate(info)?;

        // Receive server first message
        let offset = len + 1;
        let resp = if offset < BUF_SIZE && self.buf[offset] != NULL_BYTE {
            self.read_buf(offset).1
        } else {
            await!(stream.read(&mut self.buf))?; // message 4
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
        await!(stream.write_all(&msg))?; // message 5

        // Receive server final message
        await!(stream.read(&mut self.buf))?; // message 6
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
        #[allow(clippy::absurd_extreme_comparisons)]
        match serde_json::from_slice::<ServerInfo>(resp) {
            Ok(info) => {
                if !info.success {
                    let error = str::from_utf8(resp)?;
                    return Err(err::Runtime::Internal(error.to_owned()))?;
                }
                if PROTOCOL_VERSION < info.min_protocol_version
                    || info.max_protocol_version < PROTOCOL_VERSION
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
                    let error = str::from_utf8(resp)?;
                    return Err(err::Runtime::Internal(error.to_owned()))?;
                }
                Ok(info)
            }
            Err(_) => {
                let error = str::from_utf8(resp)?;
                Err(err::Driver::Other(error.to_owned()))?
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
