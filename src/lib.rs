//! ReQL is the RethinkDB query language. It offers a very powerful and
//! convenient way to manipulate JSON documents.
//!
//! # Start the server #
//!
//! ## Linux and OS X ##
//!
//! Start the server from a terminal window.
//!
//! ```bash
//! $ rethinkdb
//! ```
//!
//! ## Windows ##
//!
//! Start the server from the Windows command prompt.
//!
//! ```bash
//! C:\Path\To\RethinkDB\>rethinkdb.exe
//! ```
//!
//! # Import the driver #
//!
//! First, make sure you have `protoc` installed and in your `PATH`. See
//! [`prost-build` documentation](https://docs.rs/prost-build/0.7.0/prost_build/#sourcing-protoc)
//! for more details if it fails to compile.
//!
//! Add this crate (`reql`) and the `futures` crate to your dependencies in `Cargo.toml`.
//! You may also need to depend on a crate that provides an async implementation of a
//! `TcpStream`, like `async-std`, for example. You can use this crate
//! with any `TcpStream` that implements `AsyncRead` and `AsyncWrite` from
//! the futures crate. The implementation also needs to implement the same
//! traits for `&TcpStream` because we use unmutable references of the
//! connection to take advantage of RethinkDB's connection pipelining abilities.
//!
//! Now import the RethinkDB driver:
//!
//! ```
//! use reql::r;
//! ```
//!
//! You can now access RethinkDB commands through the [`r` struct](r).
//!
//! # Open a connection #
//!
//! When you first start RethinkDB, the server opens a port for the client
//! drivers (`28015` by default). Let's open a connection:
//!
//! ```
//! use async_std::net::TcpStream;
//! use reql::{r, DEFAULT_ADDR};
//!
//! # async fn connect() -> reql::Result<()> {
//! let stream = TcpStream::connect(DEFAULT_ADDR).await?;
//! let connection = r.connection(stream).await?;
//! # Ok(())
//! # }
//! ```
//!
//! The variable `connection` is now initialized and we can run queries.
//!
//! # Send a query to the database #
//!
//! ```
//! use async_std::net::TcpStream;
//! use futures::TryStreamExt;
//! use reql::{r, DEFAULT_ADDR};
//!
//! # async fn connect() -> reql::Result<()> {
//! let stream = TcpStream::connect(DEFAULT_ADDR).await?;
//! let conn = r.connection(stream).await?;
//! let mut query = r.expr("Hello world!").run(&conn);
//! assert_eq!(query.try_next().await?, Some("Hello world!".to_owned()));
//! # Ok(())
//! # }
//! ```
//!
//! [See the `r` struct for more available commands](r)

//#![deny(missing_docs)]

pub mod cmd;
mod err;
mod proto;

use cmd::run::Response;
use dashmap::DashMap;
use futures::channel::mpsc::Sender;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use futures::lock::Mutex;
use serde_json::Value;
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub use err::*;
pub use proto::Query;

/// Default address of the RethinkDB server
pub const DEFAULT_ADDR: (&str, u16) = ("localhost", 28015);

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

/// The connection object returned by `r.connection()`
#[derive(Debug)]
pub struct Connection<'a, T> {
    db: Cow<'a, str>,
    stream: T,
    token: AtomicU64,
    broken: AtomicBool,
    change_feed: AtomicBool,
    buffer: usize,
    senders: DashMap<u64, Sender<Result<Response>>>,
    locker: Mutex<()>,
}

impl<T> Connection<'_, T> {
    /// Convert the connection into an instance you can move around
    pub fn into_owned(self) -> Connection<'static, T> {
        Connection {
            db: Cow::from(self.db.into_owned()),
            stream: self.stream,
            token: self.token,
            broken: self.broken,
            change_feed: self.change_feed,
            buffer: self.buffer,
            senders: self.senders,
            locker: self.locker,
        }
    }

    fn mark_broken(&self) {
        self.broken.store(true, Ordering::SeqCst);
    }

    fn broken(&self) -> Result<()> {
        if self.broken.load(Ordering::SeqCst) {
            return Err(err::Client::ConnectionBroken.into());
        }
        Ok(())
    }

    fn mark_change_feed(&self) {
        self.change_feed.store(true, Ordering::SeqCst);
    }

    fn unmark_change_feed(&self) {
        self.change_feed.store(false, Ordering::SeqCst);
    }

    fn change_feed(&self) -> Result<()> {
        if self.change_feed.load(Ordering::SeqCst) {
            return Err(err::Client::ConnectionLocked.into());
        }
        Ok(())
    }
}

/// A generic, runtime independent, TcpStream
///
/// This crate can be used with any runtime that implements
/// the `AsyncRead` and the `AsyncWrite` traits from the
/// futures crate
pub trait TcpStream<'a>: AsyncReadExt + AsyncWriteExt
where
    Self: Unpin + 'a,
    &'a Self: AsyncRead + AsyncWrite,
{
}

impl<'a, T: AsyncRead + AsyncWrite + ?Sized> TcpStream<'a> for T
where
    Self: Unpin + 'a,
    &'a Self: AsyncRead + AsyncWrite,
{
}

/// The top-level ReQL namespace
///
/// # Example
///
/// Set up your top-level namespace.
///
/// ```
/// use reql::r;
/// ```
#[allow(non_camel_case_types)]
pub struct r;

impl<'a> r {
    /// Create a new connection to the database server
    ///
    /// # Example
    ///
    /// Open a connection using the default host and port, specifying the default database.
    ///
    /// ```
    /// use async_std::net::TcpStream;
    /// use reql::cmd::connection::Options;
    /// use reql::{r, DEFAULT_ADDR};
    ///
    /// # async fn connect() -> reql::Result<()> {
    /// let stream = TcpStream::connect(DEFAULT_ADDR).await?;
    /// let conn = r.connection((stream, Options::new().db("marvel"))).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read more about this command [connection]
    pub async fn connection<A, T>(self, options: A) -> Result<Connection<'a, T>>
    where
        A: cmd::connection::Arg<'a, T>,
        T: TcpStream<'a>,
        &'a T: AsyncRead + AsyncWrite,
    {
        cmd::connection::new(options.arg()).await
    }

    pub fn expr<T>(self, value: T) -> Query
    where
        T: Into<Value>,
    {
        cmd::expr::new(value.into())
    }

    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the
    /// query will run against the default database for the connection,
    /// specified in the `db` argument to [r::connection].
    ///
    /// # Examples
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```
    /// # use async_std::net::TcpStream;
    /// # use futures::TryStreamExt;
    /// # use reql::{r, DEFAULT_ADDR};
    /// # async fn example() -> reql::Result<()> {
    /// # let stream = TcpStream::connect(DEFAULT_ADDR).await?;
    /// # let conn = r.connection(stream).await?;
    /// let mut query = r.db("heroes").table("marvel").run(&conn);
    /// # assert_eq!(query.try_next().await?, Some(String::new()));
    /// # Ok(())
    /// # }
    /// ```
    pub fn db<T>(self, name: T) -> Query
    where
        T: Into<String>,
    {
        cmd::db::new(name.into())
    }

    pub fn table<T>(self, arg: T) -> Query
    where
        T: cmd::table::Arg,
    {
        cmd::table::new(None, arg.arg())
    }
}
