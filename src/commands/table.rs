//! ReQL command: table
//!
//! ## Client syntax
//!
//! > db.table(name) → table
//!
//! ## Description
//!
//! Return all documents in a table. Other commands may be chained after `table` to return a subset
//! of documents (such as [get](../get/index.html) and [filter](../filter/index.html)) or perform further processing.
//!
//! ## Example
//!
//! Return all documents in the table ‘marvel’ of the default database.
//!
//! ```norun
//! r.table("marvel").run();
//! ```
//!
//! ## Example
//!
//! Return all documents in the table ‘marvel’ of the database ‘heroes’.
//!
//! ```norun
//! r.db("heroes").table("marvel").run();
//! ```
//!
//! There are two optional arguments that may be specified.
//!
//! * `read_mode`: One of three possible values affecting the consistency guarantee for the table
//! read:
//!     * `single` returns values that are in memory (but not necessarily written to disk) on the
//!     primary replica. This is the default.
//!     * `majority` will only return values that are safely committed on disk on a majority of
//!     replicas. This requires sending a message to every replica on each read, so it is the
//!     slowest but most consistent.
//!     * `outdated` will return values that are in memory on an arbitrarily-selected replica. This
//!     is the fastest but least consistent.
//! * `identifier_format`: possible values are `name` and `uuid`, with a default of `name`. If set to
//! `uuid`, then [system tables](https://rethinkdb.com/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name.
//! (This only has an effect when used with system tables.)
//!
//! ## Example
//!
//! Allow potentially out-of-date data in exchange for faster reads.
//!
//! ```norun
//! r.db("heroes").table("marvel").read_mode(Outdated).run();
//! ```
//!
//! ## Related commands
//!
//! * [filter](../filter/index.html)
//! * [get](../get/index.html)

#![allow(dead_code)]

use ql2::types;
use types::IntoString;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use super::{TableOpts, ReadMode, IdentifierFormat};
use serde_json::value::ToJson;

impl Client<(), ()>
{
    /// Return all documents in a table. [Read more](table/index.html)
    pub fn table<T>(self, arg: T) -> Client<types::Table, TableOpts>
        where T: IntoString
    {
            let config = ::config().read();
            super::r.db(config.db()).table(arg)
    }
}

impl<O> Client<types::Db, O>
    where O: ToJson + Clone
{
    /// Return all documents in a table. [Read more](table/index.html)
    pub fn table<T>(self, arg: T) -> Client<types::Table, TableOpts>
        where T: IntoString
    {
        super::make_cmd(TermType::TABLE, Some(vec![arg.into_string()]), Some(TableOpts::default()), Some(self.cmd), self.errors)
    }
}

impl<T> Client<T, TableOpts> {
    /// Sets read mode
    pub fn read_mode(mut self, arg: ReadMode) -> Self {
        let mut opts = self.cmd.opts();
        opts.read_mode = arg;
        self.cmd.1 = Some(opts);
        self
    }

    /// Sets identifier format
    pub fn identifier_format(mut self, arg: IdentifierFormat) -> Self {
        let mut opts = self.cmd.opts();
        opts.identifier_format = arg;
        self.cmd.1 = Some(opts);
        self
    }
}
