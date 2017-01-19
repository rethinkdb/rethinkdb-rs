command! {
    /// Loop over a sequence, evaluating the given write query for each element

    #[command(for_each(func(value = "F")))]
}
