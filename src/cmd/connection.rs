//! Create a new connection to the database server
//!
//! <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/connect_javascript.png" class="api_command_illustration" />
//!
//! Accepts the following options:

use crate::cmd::debug;
use crate::{err, Connection, Result, TcpStream};
use dashmap::DashMap;
use futures::io::{AsyncRead, AsyncWrite};
use futures::lock::Mutex;
use log::trace;
use ql2::version_dummy::Version;
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str;
use std::sync::atomic::{AtomicBool, AtomicU64};

const BUF_SIZE: usize = 1024;
const NULL_BYTE: u8 = b'\0';
const PROTOCOL_VERSION: usize = 0;

/// Options accepted by [crate::r::connection]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Options<'a, T> {
    stream: T,
    /// The buffer size for each `mpsc::channel` created, by default `1024`
    buffer: usize,
    /// The database used if not explicitly specified in a query, by default `test`.
    db: &'a str,
    /// The user account to connect as (default `admin`).
    user: &'a str,
    /// The password for the user account to connect as (default `""`, empty).
    password: &'a str,
    /// Timeout period in seconds for the connection to be opened (default `20`).
    timeout: u8,
    /// A hash of options to support SSL connections (default `None`).
    /// Currently, there is only one option available, and if the `ssl` option is specified,
    /// this key is required:
    ssl: Option<Ssl<'a>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ssl<'a> {
    /// A path to the SSL CA certificate.
    ca: &'a str,
}

impl<'a, T> Options<'a, T> {
    pub const fn new(stream: T) -> Self {
        Self {
            stream,
            buffer: 1024,
            db: "test",
            user: "admin",
            password: "",
            timeout: 20,
            ssl: None,
        }
    }

    pub const fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }

    pub const fn db(mut self, db: &'a str) -> Self {
        self.db = db;
        self
    }
}

impl<'a, T> From<T> for Options<'a, T>
where
    T: TcpStream<'a>,
    &'a T: AsyncRead + AsyncWrite,
{
    fn from(stream: T) -> Self {
        Self::new(stream)
    }
}

pub(crate) async fn new<'a, T>(options: Options<'a, T>) -> Result<Connection<'a, T>>
where
    T: TcpStream<'a>,
    &'a T: AsyncRead + AsyncWrite,
{
    Ok(Connection {
        db: Cow::from(options.db),
        buffer: options.buffer,
        stream: handshake(options).await?,
        token: AtomicU64::new(0),
        broken: AtomicBool::new(false),
        senders: DashMap::new(),
        locker: Mutex::new(()),
    })
}

// Performs the actual handshake
//
// This method optimises message exchange as suggested in the RethinkDB
// documentation by sending message 3 right after message 1, without waiting
// for message 2 first.
async fn handshake<'a, T>(mut opts: Options<'_, T>) -> Result<T>
where
    T: TcpStream<'a>,
    &'a T: AsyncRead + AsyncWrite,
{
    trace!("sending supported version to RethinkDB");
    opts.stream
        .write_all(&(Version::V10 as i32).to_le_bytes())
        .await?; // message 1

    let scram = ScramClient::new(opts.user, opts.password, None);
    let (scram, msg) = client_first(scram)?;
    trace!("sending client first message");
    opts.stream.write_all(&msg).await?; // message 3

    let mut buf = [0u8; BUF_SIZE];

    trace!("receiving message(s) from RethinkDB");
    opts.stream.read(&mut buf).await?; // message 2
    let (len, resp) = bytes(&buf, 0);
    trace!("received server info; info: {}", debug(resp));
    ServerInfo::validate(resp)?;

    let offset = len + 1;
    let resp = if offset < BUF_SIZE && buf[offset] != NULL_BYTE {
        bytes(&buf, offset).1
    } else {
        trace!("reading auth response");
        opts.stream.read(&mut buf).await?; // message 4
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
    opts.stream.write_all(&msg).await?; // message 5

    trace!("reading server final message");
    opts.stream.read(&mut buf).await?; // message 6
    let resp = bytes(&buf, 0).1;
    trace!("received server final message");
    server_final(scram, resp)?;

    trace!("client connected successfully");

    Ok(opts.stream)
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
        #[allow(clippy::absurd_extreme_comparisons)]
        let info = serde_json::from_slice::<ServerInfo>(resp)?;
        if !info.success {
            return Err(err::Runtime::Internal(debug(resp)).into());
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
