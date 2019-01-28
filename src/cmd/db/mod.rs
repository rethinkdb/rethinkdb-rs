mod arg;

use crate::{r, Client};

pub use arg::Arg;

impl r {
    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the query
    /// will run against the default database for the connection, specified in
    /// the `db` argument to [connect].
    ///
    /// ## Example
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```rust
    /// # use reql::r;
    /// #
    /// r.db("heroes").table("marvel")
    /// # ;
    /// ```
    ///
    /// ## Related commands:
    ///
    /// - [table]
    /// - [db_list]
    ///
    /// [connect]: struct.r.html#method.connect
    /// [table]: struct.r.html#method.table
    /// [db_list]: struct.r.html#method.db_list
    pub fn db<A>(&self, arg: A) -> Client
    where
        A: Into<Arg>,
    {
        Client::new(&[], 14, arg.into())
    }
}
