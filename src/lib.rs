//! A native RethinkDB driver written in Rust

extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[cfg(feature = "with_io")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with_io")]
extern crate serde;
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
#[allow(unused_imports)]
extern crate reql_derive;
#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
mod macros;
mod types;
mod commands;
pub mod errors;

#[doc(hidden)]
pub use reql_derive::*;

// Needed by the `args` macro
#[doc(hidden)]
pub use ql2::proto::Term;

#[cfg(feature = "with_io")]
use std::net::SocketAddr;
use std::sync::Arc;

use errors::Error;

#[cfg(feature = "with_io")]
use std::time::Duration;
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;
#[cfg(feature = "with_io")]
use reql_io::uuid::Uuid;
#[cfg(feature = "with_io")]
use std::net::TcpStream;

use slog::Logger;

/// The result of any command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `IntoArg::into_arg`
pub struct Arg {
    string: String,
    term: Term,
    error: QueryError,
    pool: Option<Connection>,
    remote: Option<Remote>,
}

/// The response returned by the `run` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Response;

#[cfg(feature = "with_io")]
struct Session {
    id: u64,
    broken: bool,
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
struct SessionManager(Connection);

/// The connection pool returned by the `connect` command
///
/// This connection pool is designed to make it very easy
/// to pass around. It doesn't carry the actual connections
/// themselves. Instead it is simply a reference to the
/// actual underlying connection pool. As such, you can
/// `clone` or `copy` it.
#[cfg(feature = "with_io")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection(Uuid);

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

#[derive(Debug, Clone)]
enum QueryError {
    Some(Arc<errors::Error>),
    None,
}

/// The database cluster client
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Term,
    error: QueryError,
    query: String,
    logger: Logger,
}

/// The return type of the `args!()` macro
#[doc(hidden)]
pub struct Args {
    string: String,
    term: Term,
    error: QueryError,
    pool: Option<Connection>,
    remote: Option<Remote>,
}

/// The argument that is passed to any command
pub trait IntoArg {
    fn into_arg(self) -> Arg;
}

impl<T: Into<Error>> From<T> for QueryError {
    fn from(t: T) -> QueryError {
        QueryError::Some(Arc::new(t.into()))
    }
}
