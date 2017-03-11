//! A native RethinkDB driver written in Rust

extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate derive_error;
#[cfg(feature = "with_io")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "with_io")]
extern crate reql_io;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod types;
mod commands;
pub mod errors;

// Needed by the `args` macro
#[doc(hidden)]
pub use ql2::proto::Term;

#[cfg(feature = "with_io")]
use std::net::SocketAddr;
#[cfg(feature = "with_io")]
use std::sync::Arc;

use errors::Error;

#[cfg(feature = "with_io")]
use reql_io::r2d2;
#[cfg(feature = "with_io")]
use std::time::Duration;
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;
#[cfg(feature = "with_io")]
use std::net::TcpStream;

use slog::Logger;

/// The result of any command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `ToArg::to_arg`
pub struct Arg {
    string: String,
    term: Term,
    pool: Option<&'static Pool>,
    remote: Option<Remote>,
}

/// The response returned by the `run` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Response;

#[cfg(feature = "with_io")]
struct Connection {
    id: u64,
    broken: bool,
    server: Server,
    stream: TcpStream,
    logger: Logger,
}

#[cfg(feature = "with_io")]
#[derive(Clone)]
struct Config {
    cluster: Vec<Server>,
    opts: Opts,
    remote: Option<Remote>,
    logger: Logger,
}

#[cfg(feature = "with_io")]
#[derive(Debug, Clone, Copy)]
struct ConnectionManager;

/// The connection pool returned by the `connect` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Pool(r2d2::Pool<ConnectionManager>);

#[cfg(feature = "with_io")]
#[derive(Debug, Clone, Eq)]
struct Server {
    addresses: Vec<SocketAddr>,
    latency: Duration,
}

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct Opts {
    db: String,
    user: String,
    password: String,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: String,
}

/// The database cluster client
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Term,
    query: String,
    logger: Logger,
}

/// The return type of the `args!()` macro
#[doc(hidden)]
pub struct Args {
    term: Term,
    string: String,
    pool: Option<&'static Pool>,
    remote: Option<Remote>,
}

/// The argument that is passed to any command
pub trait ToArg {
    fn to_arg(&self) -> Arg;
}
