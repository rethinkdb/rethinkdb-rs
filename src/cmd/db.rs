use crate::{cmd, datum, r, Query};
use ql2::term::TermType;

impl r {
    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the
    /// query will run against the default database for the connection,
    /// specified in the `db` argument to `connect`.
    ///
    /// ## Example
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```ignore
    /// r.db("heroes").table("marvel").run(&conn).await
    /// ```
    pub fn db<T: Into<String>>(&self, name: T) -> Query {
        Query {
            term: cmd::args(TermType::Db, vec![datum::r_str(name.into())]),
        }
    }
}
