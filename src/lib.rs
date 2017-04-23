//! A native RethinkDB driver written in Rust

extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[cfg(feature = "with-io")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-io")]
extern crate serde;
#[macro_use]
extern crate derive_error;
#[cfg(feature = "with-io")]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
#[allow(unused_imports)]
extern crate reql_derive;
#[macro_use]
extern crate proc_macro_hack;
#[cfg(feature = "with-io")]
extern crate r2d2;
#[cfg(feature = "with-io")]
extern crate scram;
#[cfg(feature = "with-io")]
extern crate tokio_core;
#[cfg(feature = "with-io")]
extern crate byteorder;
#[cfg(feature = "with-io")]
extern crate futures;
#[cfg(feature = "with-io")]
extern crate parking_lot;
#[cfg(feature = "with-io")]
extern crate uuid;
#[cfg(feature = "with-io")]
extern crate ordermap;
#[cfg(feature = "with-io")]
extern crate bufstream;
extern crate chrono;

#[macro_use]
mod macros;
mod types;
mod commands;
pub mod errors;
pub mod structs;

// Needed by the `args` macro
#[doc(hidden)]
pub use reql_derive::*;
#[doc(hidden)]
pub use ql2::proto::{
    Term, Datum,
    Term_TermType as TT,
    Datum_DatumType as DT,
};
#[doc(hidden)]
pub use protobuf::repeated::RepeatedField;

#[cfg(feature = "with-io")]
use std::net::SocketAddr;

#[cfg(feature = "with-io")]
use std::time::Duration;
#[cfg(feature = "with-io")]
use tokio_core::reactor::Remote;
#[cfg(feature = "with-io")]
use uuid::Uuid;
#[cfg(feature = "with-io")]
use std::net::TcpStream;
#[cfg(feature = "with-io")]
use serde::de::DeserializeOwned;
#[cfg(feature = "with-io")]
use futures::sync::mpsc::{Receiver, Sender};
#[cfg(feature = "with-io")]
use ordermap::OrderMap;

use errors::Error;
use slog::Logger;
use serde_json::Value;

/// The result of any command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `IntoArg::into_arg`
#[derive(Clone)]
pub struct Arg {
    string: String,
    term: Result<Term>,
    pool: Option<Connection>,
    remote: Option<Remote>,
}

/// ReQL Response
///
/// Response returned by `run()`
#[cfg(feature = "with-io")]
pub type Response<T> = Receiver<Result<Option<ResponseValue<T>>>>;

#[cfg(feature = "with-io")]
struct Request<T: DeserializeOwned + Send> {
    term: Term,
    opts: Term,
    pool: r2d2::Pool<SessionManager>,
    cfg: Config,
    tx: Sender<Result<Option<ResponseValue<T>>>>,
    write: bool,
    retry: bool,
    logger: Logger,
}

#[cfg(feature = "with-io")]
struct Session {
    id: u64,
    broken: bool,
    stream: TcpStream,
    logger: Logger,
}

#[cfg(feature = "with-io")]
#[derive(Clone)]
struct Config {
    cluster: OrderMap<String, Server>,
    opts: Opts,
    remote: Remote,
    logger: Logger,
}

#[cfg(feature = "with-io")]
#[derive(Debug, Clone, Copy)]
struct SessionManager(Connection);

/// The connection pool returned by the `connect` command
///
/// This connection pool is designed to make it very easy
/// to pass around. It doesn't carry the actual connections
/// themselves. Instead it is simply a reference to the
/// actual underlying connection pool. As such, you can
/// `clone` or `copy` it.
#[cfg(feature = "with-io")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection(Uuid);

#[cfg(feature = "with-io")]
#[derive(Debug, Clone, Eq)]
struct Server {
    name: String,
    addresses: Vec<SocketAddr>,
    latency: Duration,
}

#[cfg(feature = "with-io")]
#[derive(Debug, Clone)]
struct Opts {
    db: String,
    user: String,
    password: String,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[cfg(feature = "with-io")]
#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: String,
}

/// The database cluster client
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Result<Term>,
    query: String,
    write: bool,
    logger: Logger,
}

/// Response value
#[cfg(feature = "with-io")]
#[derive(Debug, Clone)]
pub enum ResponseValue<T: DeserializeOwned + Send> {
    Expected(T),
    Unexpected(Value),
}

#[derive(Debug, Clone)]
pub struct DateTime(chrono::DateTime<chrono::UTC>);

#[derive(Serialize, Deserialize, Debug)]
#[cfg(feature = "with-io")]
struct ReqlResponse {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

/// The argument that is passed to any command
pub trait IntoArg {
    /// Converts a supported type into Arg
    fn into_arg(self) -> Arg;
}

/// Lazily execute a command
#[cfg(feature = "with-io")]
pub trait Run<A: IntoArg> {
    /// Prepare a commmand to be submitted
    fn run<T: DeserializeOwned + Send + 'static>(&self, args: A) -> Result<Response<T>>;
}
