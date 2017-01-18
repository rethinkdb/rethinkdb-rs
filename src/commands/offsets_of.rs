command! {
    /// Get the indexes of an element in a sequence. If the argument is a predicate, get the
    /// indexes of all elements matching it

    #[command(offsets_of(args(arg = "T")))]
}
