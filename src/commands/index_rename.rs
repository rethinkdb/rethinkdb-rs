command! {
    /// Rename an existing secondary index on a table

    #[command(index_rename(args(O = "old_name", N = "new_name")))]
}
