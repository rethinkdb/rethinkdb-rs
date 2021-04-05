use crate::{err, r, Connection, Result};
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const NULL_BYTE: u8 = b'\0';
const BUF_SIZE: usize = 512;
const PROTOCOL_VERSION: usize = 0;

enum Version {
    V1_0 = 0x34c2_bdc3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Options<'a> {
    pub host: &'a str,
    pub port: u16,
    pub db: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub timeout: u8,
    pub ssl: Option<Ssl<'a>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ssl<'a> {
    pub ca_certs: &'a str,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            host: "localhost",
            port: 28015,
            db: "test",
            user: "admin",
            password: "",
            timeout: 20,
            ssl: None,
        }
    }
}

impl<'a> From<&'a str> for Options<'a> {
    fn from(host: &'a str) -> Self {
        Self {
            host,
            ..Default::default()
        }
    }
}

impl<'a> From<&'a String> for Options<'a> {
    fn from(host: &'a String) -> Self {
        Self {
            host,
            ..Default::default()
        }
    }
}

impl r {
    pub async fn connect<'a, T: Into<Options<'a>>>(&self, options: T) -> Result<Connection> {
        let opts = options.into();
        let mut stream = TcpStream::connect((opts.host, opts.port)).await?;
        handshake(&mut stream, opts).await?;
        Ok(Connection { stream })
    }
}

// Performs the actual handshake
//
// This method optimises message exchange as suggested in the RethinkDB
// documentation by sending message 3 right after message 1, without waiting
// for message 2 first.
async fn handshake<'a>(stream: &mut TcpStream, opts: Options<'a>) -> Result<()> {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    // Send the version we support
    let version = (Version::V1_0 as u32).to_le_bytes();
    stream.write_all(&version).await?; // message 1

    // Send client first message
    let scram = ScramClient::new(opts.user, opts.password, None);
    let (scram, msg) = client_first(scram)?;
    stream.write_all(&msg).await?; // message 3

    // Receive supported versions
    stream.read(&mut buf).await?; // message 2
    let (len, info) = read_buf(&buf, 0);
    ServerInfo::validate(info)?;

    // Receive server first message
    let offset = len + 1;
    let resp = if offset < BUF_SIZE && buf[offset] != NULL_BYTE {
        read_buf(&buf, offset).1
    } else {
        stream.read(&mut buf).await?; // message 4
        read_buf(&buf, 0).1
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
    stream.write_all(&msg).await?; // message 5

    // Receive server final message
    stream.read(&mut buf).await?; // message 6
    server_final(scram, read_buf(&buf, 0).1)?;

    Ok(())
}

fn read_buf(buf: &[u8; BUF_SIZE], offset: usize) -> (usize, &[u8]) {
    let len = (&buf[offset..])
        .iter()
        .take_while(|x| **x != NULL_BYTE)
        .count();
    let max = offset + len;
    (max, &buf[offset..max])
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
            Err(..) => {
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
                    if let Some(10..=20) = info.error_code {
                        if let Some(msg) = info.error {
                            return Err(err::Driver::Auth(msg))?;
                        }
                    }
                    let error = str::from_utf8(resp)?;
                    return Err(err::Runtime::Internal(error.to_owned()))?;
                }
                Ok(info)
            }
            Err(..) => {
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
