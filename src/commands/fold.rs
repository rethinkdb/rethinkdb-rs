command! {
    /// Apply a function to a sequence in order, maintaining state via an accumulator

    #[command(fold(args(base = "B", function = "F")))]
}
