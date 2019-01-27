mod arg;

use crate::r;
use bytes::{BufMut, Bytes, BytesMut};

pub use arg::Arg;

#[derive(Debug, Clone)]
pub struct Db {
    pub(super) bytes: Bytes,
}

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
    pub fn db<A>(&self, arg: A) -> Db
    where
        A: Into<Arg>,
    {
        let (header, arg, footer) = ("[14,[", arg.into().bytes, "]]");
        let len = header.len() + arg.len() + footer.len();
        let mut cmd = BytesMut::with_capacity(len);
        cmd.put(header);
        cmd.put(arg);
        cmd.put(footer);
        Db {
            bytes: cmd.freeze(),
        }
    }
}
