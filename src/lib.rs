//! A native RethinkDB driver written in Rust

extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate derive_error;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
//#[allow(unused_imports)]
extern crate reql_derive;
extern crate reql_types;
#[macro_use]
extern crate proc_macro_hack;
extern crate r2d2;
extern crate scram;
extern crate tokio_core;
extern crate byteorder;
extern crate futures;
extern crate parking_lot;
extern crate uuid;
extern crate ordermap;
extern crate bufstream;

#[macro_use]
mod macros;
mod types;
mod commands;
pub mod errors;

// Needed by the `args` macro
#[doc(hidden)]
pub use reql_derive::*;
#[doc(hidden)]
pub use ql2::proto::{Term, Datum, Term_TermType as TT, Datum_DatumType as DT};
#[doc(hidden)]
pub use protobuf::repeated::RepeatedField;

use std::net::SocketAddr;
use std::time::Duration;
use std::net::TcpStream;

use tokio_core::reactor::Remote;
use uuid::Uuid;
use serde::de::DeserializeOwned;
use futures::sync::mpsc::{Receiver, Sender};
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
#[derive(Debug)]
pub struct Response<T: DeserializeOwned + Send> {
    done: bool,
    rx: Receiver<Result<Option<Document<T>>>>,
}


struct Request<T: DeserializeOwned + Send> {
    term: Term,
    opts: Term,
    pool: r2d2::Pool<SessionManager>,
    cfg: Config,
    tx: Sender<Result<Option<Document<T>>>>,
    write: bool,
    retry: bool,
    logger: Logger,
}


struct Session {
    id: u64,
    broken: bool,
    stream: TcpStream,
    logger: Logger,
}


#[derive(Clone)]
struct Config {
    cluster: OrderMap<String, Server>,
    opts: Opts,
    remote: Remote,
    logger: Logger,
}

#[derive(Debug, Clone, Copy)]
struct SessionManager(Connection);

/// The connection pool returned by the `connect` command
///
/// This connection pool is designed to make it very easy
/// to pass around. It doesn't carry the actual connections
/// themselves. Instead it is simply a reference to the
/// actual underlying connection pool. As such, you can
/// `clone` or `copy` it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection(Uuid);

#[derive(Debug, Clone, Eq)]
struct Server {
    name: String,
    addresses: Vec<SocketAddr>,
    latency: Duration,
}

#[derive(Debug, Clone)]
struct Opts {
    db: String,
    user: String,
    password: String,
    retries: u64,
    reproducible: bool,
    tls: Option<TlsCfg>,
}

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

/// The JSON document returned by the server
#[derive(Debug, Clone)]
pub enum Document<T: DeserializeOwned + Send> {
    Expected(T),
    Unexpected(Value),
}

#[derive(Serialize, Deserialize, Debug)]
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
pub trait Run<A: IntoArg> {
    /// Prepare a commmand to be submitted
    fn run<T: DeserializeOwned + Send + 'static>(&self, args: A) -> Result<Response<T>>;
}
