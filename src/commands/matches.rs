command! {
    /// Match against a regular expression
    ///
    /// This is the `match` command in the official drivers. However, `match` is a keyword in Rust.

    #[command(matches(args(regexp = "T")))]
}
