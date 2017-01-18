command! {
    /// Rename an existing secondary index on a table

    #[command(index_rename(args(old_name = "O", new_name = "N")))]
}
