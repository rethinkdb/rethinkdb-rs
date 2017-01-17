command! {
    /// Get all documents between two keys

    #[command(between(args(T = "lower_key", T = "upper_key")))]
}
