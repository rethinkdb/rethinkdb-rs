command! {
    /// Apply a function to a sequence in order, maintaining state via an accumulator

    #[command(fold(args(B = "base", F = "function")))]
}
