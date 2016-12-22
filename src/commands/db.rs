//! ReQL command: db
//!
//! ## Client syntax
//!
//! > r.db(dbName) → db
//!
//! ## Description
//!
//! Reference a database.
//!
//! The `db` command is optional. If it is not present in a query, the query will run against the
//! default database for the connection, specified in the `db` argument to [connect](../connect/index.html).
//!
//! ## Example
//!
//! Explicitly specify a database for a query.
//!
//! ```norun
//! r.db("heroes").table("marvel").run();
//! ```
//!
//! ## Related commands
//!
//! * [table](../table/index.html)
//! * [db_list](../db_list/index.html)

#![allow(dead_code)]

use ql2::types;
use types::string::IntoString;
use ql2::proto::Term_TermType as TermType;
use ::Client;

impl Client<(), ()> {
    /// Reference a database. [Read more](db/index.html)
    pub fn db<T>(self, arg: T) -> Client<types::Db, ()>
        where T: IntoString
    {
        super::make_cmd(TermType::DB, Some(vec![arg.into_string()]), None, Root!(), self.errors)
    }
}
