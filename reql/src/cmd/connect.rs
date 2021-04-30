//! Create a new connection to the database server

use super::debug;
use crate::{err, Connection, Result};
use async_net::TcpStream;
use dashmap::DashMap;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::lock::Mutex;
use log::trace;
use ql2::version_dummy::Version;
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicU64};

const BUF_SIZE: usize = 1024;
const NULL_BYTE: u8 = b'\0';
const PROTOCOL_VERSION: usize = 0;

pub(crate) const DEFAULT_DB: &str = "test";

/// Options accepted by [crate::r::connect]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Options<'a> {
    pub host: &'a str,
    pub port: u16,
    /// The database used if not explicitly specified in a query, by default `test`.
    pub db: &'a str,
    /// The user account to connect as (default `admin`).
    pub user: &'a str,
    /// The password for the user account to connect as (default `""`, empty).
    pub password: &'a str,
    /// The buffer size for each `mpsc::channel` created, by default `1024`
    pub buffer: usize,
}

impl<'a> Options<'a> {
    /// Create new options from default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the buffer size for each `mpsc::channel` created, by default `1024`
    pub const fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }

    /// Set the database used if not explicitly specified in a query, by default `test`.
    pub const fn db(mut self, db: &'a str) -> Self {
        self.db = db;
        self
    }

    /// Set the user account to connect as (default `admin`).
    pub const fn user(mut self, user: &'a str) -> Self {
        self.user = user;
        self
    }

    /// Set the password for the user account to connect as (default `""`, empty).
    pub const fn password(mut self, password: &'a str) -> Self {
        self.password = password;
        self
    }
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            host: "localhost",
            port: 28015,
            db: DEFAULT_DB,
            user: "admin",
            password: "",
            buffer: 1024,
        }
    }
}

/// The arguments accepted by [crate::r::connect]
pub trait Arg<'a> {
    fn into(self) -> Options<'a>;
}

impl<'a> Arg<'a> for () {
    fn into(self) -> Options<'a> {
        Default::default()
    }
}

impl<'a> Arg<'a> for Options<'a> {
    fn into(self) -> Self {
        self
    }
}

pub(crate) async fn new<'a>(options: Options<'a>) -> Result<Connection<'a>> {
    Ok(Connection {
        db: Cow::from(options.db),
        buffer: options.buffer,
        stream: handshake(options).await?,
        token: AtomicU64::new(0),
        broken: AtomicBool::new(false),
        change_feed: AtomicBool::new(false),
        senders: DashMap::new(),
        locker: Mutex::new(()),
    })
}

// Performs the actual handshake
//
// This method optimises message exchange as suggested in the RethinkDB
// documentation by sending message 3 right after message 1, without waiting
// for message 2 first.
async fn handshake<'a>(opts: Options<'_>) -> Result<TcpStream> {
    let mut stream = TcpStream::connect((opts.host, opts.port)).await?;

    trace!("sending supported version to RethinkDB");
    stream
        .write_all(&(Version::V10 as i32).to_le_bytes())
        .await?; // message 1

    let scram = ScramClient::new(opts.user, opts.password, None);
    let (scram, msg) = client_first(scram)?;
    trace!("sending client first message");
    stream.write_all(&msg).await?; // message 3

    let mut buf = [0u8; BUF_SIZE];

    trace!("receiving message(s) from RethinkDB");
    stream.read(&mut buf).await?; // message 2
    let (len, resp) = bytes(&buf, 0);
    trace!("received server info; info: {}", debug(resp));
    ServerInfo::validate(resp)?;

    let offset = len + 1;
    let resp = if offset < BUF_SIZE && buf[offset] != NULL_BYTE {
        bytes(&buf, offset).1
    } else {
        trace!("reading auth response");
        stream.read(&mut buf).await?; // message 4
        bytes(&buf, 0).1
    };
    trace!("received auth response");
    let info = AuthResponse::from_slice(resp)?;
    let auth = match info.authentication {
        Some(auth) => auth,
        None => {
            let msg = String::from("server did not send authentication info");
            return Err(err::Client::Other(msg).into());
        }
    };

    let (scram, msg) = client_final(scram, &auth)?;
    trace!("sending client final message");
    stream.write_all(&msg).await?; // message 5

    trace!("reading server final message");
    stream.read(&mut buf).await?; // message 6
    let resp = bytes(&buf, 0).1;
    trace!("received server final message");
    server_final(scram, resp)?;

    trace!("client connected successfully");

    Ok(stream)
}

fn bytes(buf: &[u8], offset: usize) -> (usize, &[u8]) {
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

impl ServerInfo<'_> {
    fn validate(resp: &[u8]) -> Result<()> {
        let info = serde_json::from_slice::<ServerInfo>(resp)?;
        if !info.success {
            return Err(err::Runtime::Internal(debug(resp)).into());
        }
        #[allow(clippy::absurd_extreme_comparisons)]
        if PROTOCOL_VERSION < info.min_protocol_version
            || info.max_protocol_version < PROTOCOL_VERSION
        {
            let msg = format!(
                "unsupported protocol version {version}, expected between {min} and {max}",
                version = PROTOCOL_VERSION,
                min = info.min_protocol_version,
                max = info.max_protocol_version,
            );
            return Err(err::Client::Other(msg).into());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: usize,
    authentication_method: &'static str,
    authentication: String,
}

fn client_first(scram: ScramClient<'_>) -> Result<(ServerFirst<'_>, Vec<u8>)> {
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

fn client_final(scram: ServerFirst<'_>, auth: &str) -> Result<(ServerFinal, Vec<u8>)> {
    let scram = scram
        .handle_server_first(auth)
        .map_err(|x| x.to_string())
        .map_err(err::Client::Other)?;
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
        let info = serde_json::from_slice::<AuthResponse>(resp)?;
        if !info.success {
            // If error code is between 10 and 20, this is an auth error
            if let Some(10..=20) = info.error_code {
                if let Some(msg) = info.error {
                    return Err(err::Client::Auth(msg).into());
                }
            }
            return Err(err::Runtime::Internal(debug(resp)).into());
        }
        Ok(info)
    }
}

fn server_final(scram: ServerFinal, resp: &[u8]) -> Result<()> {
    let info = AuthResponse::from_slice(resp)?;
    if let Some(auth) = info.authentication {
        if let Err(error) = scram.handle_server_final(&auth) {
            return Err(err::Client::Other(error.to_string()).into());
        }
    }
    Ok(())
}
