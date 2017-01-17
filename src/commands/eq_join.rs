command! {
    /// Join tables using a field or function on the left-hand sequence matching primary keys or
    /// secondary indexes on the right-hand table

    #[eq_join(args(L = "left_arg", R = "right_table"))]
}
