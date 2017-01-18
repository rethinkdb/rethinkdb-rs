command! {
    /// Join tables using a field or function on the left-hand sequence matching primary keys or
    /// secondary indexes on the right-hand table

    #[command(eq_join(args(left_arg = "L", right_table = "R")))]
}
