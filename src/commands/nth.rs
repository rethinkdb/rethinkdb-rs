command! {
    /// Get the nth element of a sequence, counting from zero. If the argument is negative, count
    /// from the last element

    #[command(nth(args(index = "T")))]
}
