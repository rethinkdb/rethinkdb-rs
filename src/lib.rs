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

use std::net::SocketAddr;

use errors::Error;

#[cfg(feature = "with_io")]
use reql_io::r2d2;

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
pub struct Response<T>(T);

/// The ReQL connection returned by the `connect` command
///
/// Internally this is actually a connection pool.
pub struct Connection;

/// The configuration data for the `connect` command
#[derive(Debug)]
pub struct Config(Vec<InnerConfig>);

#[derive(Debug)]
struct InnerConfig {
    pool: r2d2::Config<Connection, Error>,
    addresses: Vec<SocketAddr>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: &'static str,
}

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Client {
    term: Term,
    query: String,
    logger: Logger,
}

impl Arg {
    #[doc(hidden)]
    pub fn term(self) -> Term {
        self.term
    }
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
