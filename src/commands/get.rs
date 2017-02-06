command! {
    /// Get a document by primary key
    ///
    /// If no document exists with that primary key, `get` will return `None`.
    ///
    /// # Example
    ///
    /// Find a document by UUID.
    ///
    /// ```reql
    /// r.table("posts").get("a9849eef-7176-4411-935b-79a6e3c56a74");
    /// ```
    ///
    /// # Example
    ///
    /// Find a document and merge another document with it.
    ///
    /// ```reql
    /// r.table("heroes").get(3).merge(args!({powers: ["invisibility", "speed"]}));
    /// ```
    ///
    /// # Example
    ///
    /// Subscribe to a document’s [changefeed](https://rethinkdb.com/docs/changefeeds/ruby/).
    ///
    /// ```reql
    /// r.table("heroes").get(3).changes();
    /// ```

    #[command(
        get(
            args(key = "T"),
            related(get_all, between),
        )
    )]
}
