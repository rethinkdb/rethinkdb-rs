command! {
    /// Return all the elements in a sequence for which the given predicate is true
    ///
    /// The return value of `filter` will be the same as the input (sequence, stream, or array).
    /// Documents can be filtered in a variety of ways—ranges, nested values, boolean conditions,
    /// and the results of anonymous functions.
    ///
    /// By default, `filter` will silently skip documents with missing fields: if the predicate tries
    /// to access a field that doesn’t exist (for instance, the predicate `{age: 30}` applied to a
    /// document with no `age` field), that document will not be returned in the result set, and no
    /// error will be generated. This behavior can be changed with the `default` optional argument.
    ///
    /// * If `default` is set to true, documents with missing fields will be returned rather than
    /// skipped.
    /// * If `default` is set to `r.error()`, a `RuntimeError` will be thrown when a document with a
    /// missing field is tested.
    /// * If `default` is set to `false` (the default), documents with missing fields will be skipped.
    ///
    /// > **Note:** `filter` does not use secondary indexes. For retrieving documents via secondary
    /// indexes, consider [get_all](trait.GetAll.html), [between](trait.Between.html) and
    /// [eq_join](trait.EqJoin.html).
    ///
    /// # Basic predicates
    ///
    /// # Example
    ///
    /// Get all users who are 30 years old.
    ///
    /// ```reql
    /// r.table("users").filter(args!({age: 30}));
    /// ```
    ///
    /// The predicate `{age: 30}` selects documents in the `users` table with an `age` field whose value
    /// is `30`. Documents with an `age` field set to any other value or with no `age` field present are
    /// skipped.
    ///
    /// While the `{field: value}` style of predicate is useful for exact matches, a more general
    /// way to write a predicate is to use a Rust closure that returns `true` or `false`.
    ///
    /// ```reql
    /// r.table("users").filter(args!(|user| user.get_field("age").eq(30)));
    /// ```
    ///
    /// In this case, the function returns `true` if the field `age` is equal to 30. Predicates to
    /// `filter` are evaluated on the server, and must use ReQL expressions. Also, predicates must
    /// evaluate document fields. They cannot evaluate [secondary
    /// indexes](https://rethinkdb.com/docs/secondary-indexes/java/).
    ///
    /// # Example
    ///
    /// Get all users who are more than 18 years old.
    ///
    /// ```reql
    /// r.table("users").filter(args!(|user| user.get_field("age").gt(18)));
    /// ```
    ///
    /// # Example
    ///
    /// Get all users who are less than 18 years old and more than 13 years old.
    ///
    /// ```reql
    /// r.table("users").filter(args!(|user| {
    ///     let age = user.get_field("age");
    ///     age.lt(18).and(age.gt(13))
    /// }));
    /// ```

    #[command(filter(args(args = "T")))]
}
