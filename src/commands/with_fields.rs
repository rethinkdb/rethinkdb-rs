command! {
    /// Plucks one or more attributes from a sequence of objects, filtering out any objects in the
    /// sequence that do not have the specified fields

    #[command(with_fields(args(T = "fields")))]
}
