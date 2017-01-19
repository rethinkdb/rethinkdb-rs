command! {
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
    /// # use reql::commands::run::Dummy;
    /// # use reql::r;
    /// # struct Heroes;
    /// # fn main() {
    /// r.db("heroes").table("marvel").run::<Heros>();
    /// # }
    /// ```

    #[command(
        db(
            args(name = "T"),
            related(table, db_list),
        )
    )]
}
