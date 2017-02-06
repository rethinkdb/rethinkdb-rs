command! {
    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the query will run against the
    /// default database for the connection, specified in the `db` argument to `connect`.
    ///
    /// # Example
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```reql
    /// r.db("heroes").table("marvel");
    /// ```

    #[command(
        db(
            args(name = "T"),
            related(table, db_list),
        )
    )]
}
