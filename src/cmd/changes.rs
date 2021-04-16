//! Turn a query into a changefeed, an infinite stream of objects
//! representing changes to the query's results as they occur
//!
//! A changefeed may return changes to a table or an individual document
//! (a "point" changefeed). Commands such as `filter` or `map` may be used
//! before the `changes` command to transform or filter the output, and
//! many commands that operate on sequences can be chained after `changes`.
//!
//! If the table becomes unavailable, the changefeed will be disconnected,
//! and a runtime exception will be thrown by the driver.
//!
//! Changefeed notifications take the form of a two-field object:
//!
//! ```js
//! {
//!    "old_val": <document before change>,
//!    "new_val": <document after change>
//! }
//! ```
//!
//! When `include_types` is `true`, there will be three fields:
//!
//! ```js
//! {
//!    "old_val": <document before change>,
//!    "new_val": <document after change>,
//!    "type": <result type>
//! }
//! ```
//!
//! When a document is deleted, `new_val` will be `null`; when a document is
//! inserted, `old_val` will be `null`.
//!
//! Certain document transformation commands can be chained before changefeeds.
//! For more information, read the [discussion of changefeeds](https://rethinkdb.com/docs/changefeeds/)
//! in the "Query language" documentation.
//!
//! Changefeeds ignore the `read_mode` flag to `run`, and always behave as if
//! it is set to `single` (i.e., the values they return are in memory on the primary
//! replica, but have not necessarily been written to disk yet). For more details
//! read [Consistency guarantees](https://rethinkdb.com/docs/consistency).
//!
//! The server will buffer up to `changefeed_queue_size` elements (default 100,000).
//! If the buffer limit is hit, early changes will be discarded, and the client will
//! receive an object of the form
//! `{error: "Changefeed cache over array size limit, skipped X elements."}`
//! where `X` is the number of elements skipped.
//!
//! Commands that operate on streams (such as [filter](super::filter) or [map](super::map))
//! can usually be chained after `changes`.  However, since the stream produced by
//! `changes` has no ending, commands that need to consume the entire stream before
//! returning (such as [reduce](super::reduce) or [count](super::count)) cannot.
//!
//! # Examples
//!
//! Subscribe to the changes on a table.
//!
//! Start monitoring the changefeed in one client:
//!
//! ```
//! # use async_std::net::TcpStream;
//! # use futures::TryStreamExt;
//! # use reql::{r, DEFAULT_ADDR};
//! # use serde_json::Value;
//! # async fn example() -> reql::Result<()> {
//! # let stream = TcpStream::connect(DEFAULT_ADDR).await?;
//! # let conn = r.connection(stream).await?;
//! let mut query = r.table("games").changes(()).run(&conn);
//!
//! while let Some(response) = query.try_next().await? {
//!     print_value(response);
//! }
//! # Ok(())
//! # }
//! # fn print_value(value: Value) {
//! # println!("{}", value);
//! # }
//! ```
//!
//! As these queries are performed in a second client
//!
//! ```
//! # use async_std::net::TcpStream;
//! # use futures::TryStreamExt;
//! # use reql::{r, DEFAULT_ADDR};
//! # use serde_json::{Value, json};
//! # async fn example() -> reql::Result<()> {
//! # let stream = TcpStream::connect(DEFAULT_ADDR).await?;
//! # let conn = r.connection(stream).await?;
//! let mut query = r.table("games").insert(json!({"id": 1})).run(&conn);
//! #
//! # if let Some(response) = query.try_next().await? {
//! #    print_value(response);
//! # }
//! # Ok(())
//! # }
//! # fn print_value(value: Value) {
//! # println!("{}", value);
//! # }
//! ```
//!
//! the first client would receive and print the following objects:
//!
//! ```json
//! {old_val: null, new_val: {id: 1}}
//! ```

use crate::Query;
use ql2::term::TermType;
use serde::Serialize;

/// Optional arguments to `changes`
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    /// Controls how change notifications are batched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<Squash>,
    /// The number of changes the server will buffer between client reads
    /// before it starts dropping changes and generates an error
    /// (default: 100,000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changefeed_queue_size: Option<u32>,
    /// If `true`, the changefeed stream will begin with the current contents
    /// of the table or selection being monitored. These initial results will
    /// have `new_val` fields, but no `old_val` fields. The initial results
    /// may be intermixed with actual changes, as long as an initial result
    /// for the changed document has already been given. If an initial result
    /// for a document has been sent and a change is made to that document
    /// that would move it to the unsent part of the result set (e.g., a
    /// changefeed monitors the top 100 posters, the first 50 have been sent,
    /// and poster 48 has become poster 52), an "uninitial" notification will
    /// be sent, with an `old_val` field but no `new_val` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_initial: Option<bool>,
    /// If `true`, the changefeed stream will include special status documents
    /// consisting of the field `state` and a string indicating a change in the
    /// feed's state. These documents can occur at any point in the feed between
    /// the notification documents described below. If `includeStates` is `false`
    /// (the default), the status documents will not be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_states: Option<bool>,
    /// If `true`, a changefeed stream on an `order_by.limit` changefeed will
    /// include `old_offset` and `new_offset` fields in status documents that
    /// include `old_val` and `new_val`. This allows applications to maintain
    /// ordered lists of the stream's result set. If `old_offset` is set and not
    /// `null`, the element at `old_offset` is being deleted; if `new_offset` is
    /// set and not `null`, then `new_val` is being inserted at `new_offset`.
    /// Setting `include_offsets` to `true` on a changefeed that does not support
    /// it will raise an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_offsets: Option<bool>,
    /// If `true`, every result on a changefeed will include a `type` field with
    /// a string that indicates the kind of change the result represents:
    /// `add`, `remove`, `change`, `initial`, `uninitial`, `state`.
    /// Defaults to `false`.
    ///
    /// There are currently two states:
    ///
    /// * `{state: 'initializing'}` indicates the following documents represent
    /// initial values on the feed rather than changes. This will be the first
    /// document of a feed that returns initial values.
    /// * `{state: 'ready'}` indicates the following documents represent changes.
    /// This will be the first document of a feed that does *not* return initial
    /// values; otherwise, it will indicate the initial values have all been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_types: Option<bool>,
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub const fn squash(mut self, squash: Squash) -> Self {
        self.squash = Some(squash);
        self
    }

    pub const fn changefeed_queue_size(mut self, changefeed_queue_size: u32) -> Self {
        self.changefeed_queue_size = Some(changefeed_queue_size);
        self
    }

    pub const fn include_initial(mut self, include_initial: bool) -> Self {
        self.include_initial = Some(include_initial);
        self
    }

    pub const fn include_states(mut self, include_states: bool) -> Self {
        self.include_states = Some(include_states);
        self
    }

    pub const fn include_offsets(mut self, include_offsets: bool) -> Self {
        self.include_offsets = Some(include_offsets);
        self
    }

    pub const fn include_types(mut self, include_types: bool) -> Self {
        self.include_types = Some(include_types);
        self
    }
}

/// Controls how change notifications are batched
#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Squash {
    /// `true`: When multiple changes to the same document occur before a
    /// batch of notifications is sent, the changes are "squashed" into one
    /// change. The client receives a notification that will bring it fully
    /// up to date with the server.
    /// `false`: All changes will be sent to the client verbatim. This is
    /// the default.
    Bool(bool),
    /// `n`: A numeric value (floating point). Similar to `true`, but the
    /// server will wait `n` seconds to respond in order to squash as many
    /// changes together as possible, reducing network traffic. The first
    /// batch will always be returned immediately.
    Float(f32),
}

pub trait Arg {
    fn arg(self) -> Option<Options>;
}

impl Arg for () {
    fn arg(self) -> Option<Options> {
        None
    }
}

impl Arg for Options {
    fn arg(self) -> Option<Options> {
        Some(self)
    }
}

pub(crate) fn new(parent: Query, opts: Option<Options>) -> Query {
    parent
        .append(TermType::Changes)
        .with_opts(opts)
        .mark_change_feed()
}
