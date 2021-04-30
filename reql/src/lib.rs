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
//! use reql::r;
//!
//! # async fn connect() -> reql::Result<()> {
//! let connection = r.connect(()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! The variable `connection` is now initialized and we can run queries.
//!
//! # Send a query to the database #
//!
//! ```
//! use futures::TryStreamExt;
//! use reql::r;
//!
//! # async fn connect() -> reql::Result<()> {
//! let conn = r.connect(()).await?;
//! let mut query = r.expr("Hello world!").run(&conn);
//! # let _: Option<String> = query.try_next().await?;
//! # Ok(())
//! # }
//! ```
//!
//! [See the `r` struct for more available commands](r)

#![allow(clippy::wrong_self_convention)]

pub mod cmd;
mod err;
mod proto;

use access_queue::AccessQueue;
use async_net::TcpStream;
use cmd::run::Response;
use dashmap::DashMap;
use futures::channel::mpsc::UnboundedSender;
use ql2::response::ResponseType;
use ql2::term::TermType;
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub use err::*;
pub use proto::Query;

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

/// The connection object returned by `r.connect()`
#[derive(Debug)]
pub struct Connection<'a> {
    db: Cow<'a, str>,
    stream: AccessQueue<TcpStream>,
    channels: DashMap<u64, UnboundedSender<Result<(ResponseType, Response)>>>,
    token: AtomicU64,
    broken: AtomicBool,
    change_feed: AtomicBool,
}

impl Connection<'_> {
    /// Convert the connection into an instance you can move around
    pub fn into_owned(self) -> Connection<'static> {
        Connection {
            db: Cow::from(self.db.into_owned()),
            stream: self.stream,
            channels: self.channels,
            token: self.token,
            broken: self.broken,
            change_feed: self.change_feed,
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
    /// use reql::cmd::connect::Options;
    /// use reql::r;
    ///
    /// # async fn connect() -> reql::Result<()> {
    /// let conn = r.connect(Options::new().db("marvel")).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read more about this command [cmd::connect]
    pub async fn connect<T>(self, options: T) -> Result<Connection<'a>>
    where
        T: cmd::connect::Arg<'a>,
    {
        cmd::connect::new(options.into()).await
    }

    pub fn db_create<T>(self, arg: T) -> Query
    where
        T: cmd::db_create::Arg,
    {
        arg.into_query()
    }

    pub fn db_drop<T>(self, arg: T) -> Query
    where
        T: cmd::db_drop::Arg,
    {
        arg.into_query()
    }

    pub fn db_list(self) -> Query {
        Query::new(TermType::DbList)
    }

    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the
    /// query will run against the default database for the connection,
    /// specified in the `db` argument to [r::connect].
    ///
    /// # Examples
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```
    /// # use futures::TryStreamExt;
    /// # use reql::r;
    /// # async fn example() -> reql::Result<()> {
    /// # let conn = r.connect(()).await?;
    /// let mut query = r.db("heroes").table("marvel").run(&conn);
    /// # let _: Option<String> = query.try_next().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn db<T>(self, arg: T) -> Query
    where
        T: cmd::db::Arg,
    {
        arg.into_query()
    }

    /// See [Query::table_create]
    pub fn table_create<T>(self, arg: T) -> Query
    where
        T: cmd::table_create::Arg,
    {
        arg.into_query()
    }

    pub fn table<T>(self, arg: T) -> Query
    where
        T: cmd::table::Arg,
    {
        arg.into_query()
    }

    pub fn map<T>(self, arg: T) -> Query
    where
        T: cmd::map::Arg,
    {
        arg.into_query()
    }

    pub fn union<T>(self, arg: T) -> Query
    where
        T: cmd::union::Arg,
    {
        arg.into_query()
    }

    pub fn group<T>(self, arg: T) -> Query
    where
        T: cmd::group::Arg,
    {
        arg.into_query()
    }

    pub fn reduce<T>(self, arg: T) -> Query
    where
        T: cmd::reduce::Arg,
    {
        arg.into_query()
    }

    pub fn count<T>(self, arg: T) -> Query
    where
        T: cmd::count::Arg,
    {
        arg.into_query()
    }

    pub fn sum<T>(self, arg: T) -> Query
    where
        T: cmd::sum::Arg,
    {
        arg.into_query()
    }

    pub fn avg<T>(self, arg: T) -> Query
    where
        T: cmd::avg::Arg,
    {
        arg.into_query()
    }

    pub fn min<T>(self, arg: T) -> Query
    where
        T: cmd::min::Arg,
    {
        arg.into_query()
    }

    pub fn max<T>(self, arg: T) -> Query
    where
        T: cmd::max::Arg,
    {
        arg.into_query()
    }

    pub fn distinct<T>(self, arg: T) -> Query
    where
        T: cmd::distinct::Arg,
    {
        arg.into_query()
    }

    pub fn contains<T>(self, arg: T) -> Query
    where
        T: cmd::contains::Arg,
    {
        arg.into_query()
    }

    pub fn literal<T>(self, arg: T) -> Query
    where
        T: cmd::literal::Arg,
    {
        arg.into_query()
    }

    pub fn object<T>(self, arg: T) -> Query
    where
        T: cmd::object::Arg,
    {
        arg.into_query()
    }

    pub fn random<T>(self, arg: T) -> Query
    where
        T: cmd::random::Arg,
    {
        arg.into_query()
    }

    pub fn round<T>(self, arg: T) -> Query
    where
        T: cmd::round::Arg,
    {
        arg.into_query()
    }

    pub fn ceil<T>(self, arg: T) -> Query
    where
        T: cmd::ceil::Arg,
    {
        arg.into_query()
    }

    pub fn floor<T>(self, arg: T) -> Query
    where
        T: cmd::floor::Arg,
    {
        arg.into_query()
    }

    pub fn now(self) -> Query {
        Query::new(TermType::Now)
    }

    pub fn time<T>(self, arg: T) -> Query
    where
        T: cmd::time::Arg,
    {
        arg.into_query()
    }

    pub fn epoch_time<T>(self, arg: T) -> Query
    where
        T: cmd::epoch_time::Arg,
    {
        arg.into_query()
    }

    pub fn iso8601<T>(self, arg: T) -> Query
    where
        T: cmd::iso8601::Arg,
    {
        arg.into_query()
    }

    pub fn r#do<T>(self, arg: T) -> Query
    where
        T: cmd::r#do::Arg,
    {
        arg.into_query()
    }

    pub fn branch<T>(self, arg: T) -> Query
    where
        T: cmd::branch::Arg,
    {
        arg.into_query()
    }

    pub fn range<T>(self, arg: T) -> Query
    where
        T: cmd::range::Arg,
    {
        arg.into_query()
    }

    pub fn error<T>(self, arg: T) -> Query
    where
        T: cmd::error::Arg,
    {
        arg.into_query()
    }

    pub fn expr<T>(self, arg: T) -> Query
    where
        T: cmd::expr::Arg,
    {
        arg.into_query()
    }

    pub fn js<T>(self, arg: T) -> Query
    where
        T: cmd::js::Arg,
    {
        arg.into_query()
    }

    pub fn info<T>(self, arg: T) -> Query
    where
        T: cmd::info::Arg,
    {
        arg.into_query()
    }

    pub fn json<T>(self, arg: T) -> Query
    where
        T: cmd::json::Arg,
    {
        arg.into_query()
    }

    pub fn http<T>(self, arg: T) -> Query
    where
        T: cmd::http::Arg,
    {
        arg.into_query()
    }

    pub fn uuid<T>(self, arg: T) -> Query
    where
        T: cmd::uuid::Arg,
    {
        arg.into_query()
    }

    pub fn circle<T>(self, arg: T) -> Query
    where
        T: cmd::circle::Arg,
    {
        arg.into_query()
    }

    pub fn distance<T>(self, arg: T) -> Query
    where
        T: cmd::distance::Arg,
    {
        arg.into_query()
    }

    pub fn geojson<T>(self, arg: T) -> Query
    where
        T: cmd::geojson::Arg,
    {
        arg.into_query()
    }

    pub fn intersects<T>(self, arg: T) -> Query
    where
        T: cmd::intersects::Arg,
    {
        arg.into_query()
    }

    pub fn line<T>(self, arg: T) -> Query
    where
        T: cmd::line::Arg,
    {
        arg.into_query()
    }

    pub fn point<T>(self, arg: T) -> Query
    where
        T: cmd::point::Arg,
    {
        arg.into_query()
    }

    pub fn polygon<T>(self, arg: T) -> Query
    where
        T: cmd::polygon::Arg,
    {
        arg.into_query()
    }

    pub fn grant<T>(self, arg: T) -> Query
    where
        T: cmd::grant::Arg,
    {
        arg.into_query()
    }

    pub fn wait<T>(self, arg: T) -> Query
    where
        T: cmd::wait::Arg,
    {
        arg.into_query()
    }
}
