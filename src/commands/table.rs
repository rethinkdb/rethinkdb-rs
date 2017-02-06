command! {
    /// Return all documents in a table
    ///
    /// Other commands may be chained after `table` to return a subset of documents (such as [get](trait.Get.html) and
    /// [filter](trait.Filter.html)) or perform further processing.
    ///
    /// # Example
    ///
    /// Return all documents in the table ‘marvel’ of the default database.
    ///
    /// ```reql
    /// r.table("marvel");
    /// ```
    ///
    /// # Example
    ///
    /// Return all documents in the table ‘marvel’ of the database ‘heroes’.
    ///
    /// ```reql
    /// r.db("heroes").table("marvel");
    /// ```
    ///
    /// There are two [args](../macro.args.html) that may be specified.
    ///
    /// * `read_mode`: One of three possible values affecting the consistency guarantee for the table
    /// read:
    ///     - `single` returns values that are in memory (but not necessarily written to disk) on the
    /// primary replica. This is the default.
    ///     - `majority` will only return values that are safely committed on disk on a majority of
    /// replicas. This requires sending a message to every replica on each read, so it is the
    /// slowest but most consistent.
    ///     - `outdated` will return values that are in memory on an arbitrarily-selected replica. This
    /// is the fastest but least consistent.
    /// * `identifier_format`: possible values are `name` and `uuid`, with a default of `name`. If set to
    /// `uuid`, then [system tables](https://rethinkdb.com/docs/system-tables/) will refer to servers, databases and tables by UUID rather than
    /// name. (This only has an effect when used with system tables.)
    ///
    /// # Example
    ///
    /// Allow potentially out-of-date data in exchange for faster reads.
    ///
    /// ```reql
    /// r.db("heroes").table(args!("marvel", {read_mode: "outdated"}));
    /// ```

    #[command(
        table(
            args(args = "T"),
            related(filter, get),
        )
    )]
}
