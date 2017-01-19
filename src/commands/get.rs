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
    /// # struct Post;
    /// r.table("posts").get("a9849eef-7176-4411-935b-79a6e3c56a74").run::<Post>();
    /// ```
    ///
    /// # Example
    ///
    /// Find a document and merge another document with it.
    ///
    /// ```reql,macros
    /// # struct Hero;
    /// # fn main() {
    /// r.table("heroes").get(3).merge(obj! {
    ///     powers: arr!["invisibility", "speed"],
    /// }).run::<Hero>();
    /// # }
    /// ```
    ///
    /// # Example
    ///
    /// Subscribe to a document’s [changefeed](https://rethinkdb.com/docs/changefeeds/ruby/).
    ///
    /// ```reql
    /// # struct Hero;
    /// r.table("heroes").get(3).changes().run::<Hero>();
    /// ```

    #[command(
        get(
            args(key = "T"),
            related(get_all, between),
        )
    )]
}
