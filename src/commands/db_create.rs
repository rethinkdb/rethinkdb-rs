command! {
    /// Create a database. A RethinkDB database is a collection of tables, similar to relational
    /// databases

    #[db_create(args(T = "name"))]
}
