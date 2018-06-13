//! A native ReQL driver written in Rust

extern crate bufstream;
extern crate byteorder;
#[macro_use]
extern crate derive_error;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate indexmap;
extern crate parking_lot;
extern crate protobuf;
extern crate ql2;
extern crate r2d2;
extern crate reql_types;
extern crate scram;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate uuid;
#[cfg(feature = "tls")]
extern crate native_tls;

#[macro_use]
mod macros;
mod commands;
pub mod errors;
mod types;

use errors::Error;
use indexmap::IndexMap;
#[doc(hidden)]
pub use protobuf::repeated::RepeatedField;
#[doc(hidden)]
pub use ql2::proto::{Datum, Datum_DatumType as DT, Term, Term_TermType as TT};
use serde::de::DeserializeOwned;
use serde_json::Value;
#[cfg(feature = "tls")]
use native_tls::TlsConnectorBuilder;

use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

use uuid::Uuid;
use futures::sync::mpsc::{Sender, Receiver};

/// Default ReQL port
pub const DEFAULT_PORT: u16 = 28015;

/// The result of any command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `IntoArg::into_arg`
#[derive(Clone)]
pub struct Arg {
    string: String,
    term: Result<Term>,
    pool: Option<Connection>,
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
    cfg: InnerConfig,
    tx: Sender<Result<Option<Document<T>>>>,
    write: bool,
    retry: bool,
}

struct Session {
    id: u64,
    broken: bool,
    stream: TcpStream,
}

/// Connection parameters
#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub servers: Vec<SocketAddr>,
    pub db: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    // May be changed to a timeout in future
    // See comment on Default impl
    retries: u64,
    #[cfg(feature = "tls")]
    pub tls: Option<TlsConnectorBuilder>,
}

#[derive(Clone)]
struct InnerConfig {
    cluster: IndexMap<String, Server>,
    opts: Opts,
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
    #[cfg(feature = "tls")]
    tls: Option<TlsConnectorBuilder>,
}

/// The database cluster client
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Result<Term>,
    query: String,
    write: bool,
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
