//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate derive_error;
#[cfg(feature = "with_io")]
extern crate reql_io;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod types;
pub mod commands;
pub mod errors;

#[doc(hidden)]
pub use ql2::proto::Term;

#[cfg(feature = "with_io")]
use std::net::SocketAddr;

use errors::Error;

#[cfg(feature = "with_io")]
use reql_io::r2d2;
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;

use slog::Logger;

/// The result of any ReQL command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `ToArg::to_arg`
///
/// It's not meant to be used directly.
pub struct Arg {
    string: String,
    term: Term,
}

/// The response returned by the `run` command
#[cfg(feature = "with_io")]
pub struct Response<T>(T);

/// The ReQL connection returned by the `connect` command
///
/// Internally this is actually a connection pool.
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Connection(Vec<r2d2::Pool<ConnectionManager>>);

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct InnerConnection;

#[cfg(feature = "with_io")]
#[derive(Clone)]
struct ConnectionManager {
    opts: Opts,
    remote: Remote,
}

/// The configuration data for the `connect` command
#[cfg(feature = "with_io")]
#[derive(Debug)]
pub struct Config(Vec<InnerConfig>);

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct Opts {
    addresses: Vec<SocketAddr>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[cfg(feature = "with_io")]
#[derive(Debug)]
struct InnerConfig {
    pool: r2d2::Config<InnerConnection, Error>,
    opts: Opts,
}

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: &'static str,
}

/// The type returned by every error
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Term,
    query: String,
    logger: Logger,
}

/// The return type of the `args!()` macro
#[derive(Debug, Clone)]
pub struct Args {
    term: Term,
    string: String,
}

/// The argument that is passed to any ReQL command
pub trait ToArg {
    fn to_arg(&self) -> Arg;
}
