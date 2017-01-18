command! {
    /// Create a database. A RethinkDB database is a collection of tables, similar to relational
    /// databases

    #[command(db_create(args(name = "T")))]
}
