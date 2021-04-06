mod cmd;
mod err;
mod proto;

use ql2::Term;
use std::borrow::Cow;
use tokio::net::TcpStream;

pub(crate) use proto::*;

pub use cmd::*;
pub use err::*;

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

/// The top-level ReQL namespace
#[allow(non_camel_case_types)]
pub struct r;

/// Returned by all the other commands except `changes`
#[derive(Debug, Clone)]
pub struct Query {
    term: Term,
}

/// Returned by the `changes` command
#[derive(Debug, Clone)]
pub struct Stream {
    term: Term,
}

/// The connection object returned by `r.connect()`
#[derive(Debug)]
pub struct Connection<'a> {
    db: Cow<'a, str>,
    stream: TcpStream,
}

impl Connection<'_> {
    pub fn into_owned(self) -> Connection<'static> {
        Connection {
            db: Cow::from(self.db.into_owned()),
            stream: self.stream,
        }
    }
}
