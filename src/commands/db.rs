/// Reference a database
///
/// The `db` command is optional. If it is not present in a query, the query will run against the
/// default database for the connection, specified in the `db` argument to `connect`.
///
/// ## Example
///
/// Explicitly specify a database for a query.
///
/// ```rust,norun
/// # extern crate reql;
/// # use reql::commands::*;
/// # use reql::r;
/// # fn main() {
/// let marvel = r.db("heroes").table("marvel");
/// # }
/// ```
///
/// ## Related commands
/// 
/// * [table](trait.Table.html#tymethod.table)
/// * [db_list](trait.DbList.html#tymethod.db_list)
#[derive(Command)]
enum _Db {
    ArgnameName,
}
