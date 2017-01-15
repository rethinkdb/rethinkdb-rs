/// Reference a database
///
/// The `db` command is optional. If it is not present in a query, the query will run against the
/// default database for the connection, specified in the `db` argument to `connect`.
#[derive(Command)]
enum _Db {
    ArgnameName,
}
