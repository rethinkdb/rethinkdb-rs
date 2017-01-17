command! {
    /// Pluck out one or more attributes from either an object or a sequence of objects
    /// (projection)

    #[command(pluck(args(T = "selectors")))]
}
