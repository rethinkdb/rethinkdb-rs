macro_rules! with_args {
    ( $cmd:ident, $args:ident ) => {{
        let mut tmp_args = $args;
        if let Ok(ref mut term) = $cmd.term {
            if tmp_args.has_field_type() { // did not come from the args macro
                term.mut_args().push(tmp_args);
            } else { // came from the args macro
                for arg in tmp_args.take_args().into_vec() {
                    term.mut_args().push(arg);
                }
                for pair in tmp_args.take_optargs().into_vec() {
                    term.mut_optargs().push(pair);
                }
            }
        }
    }}
}

proc_macro_expr_decl!(
    /// Splice an array of arguments into another term
    ///
    /// `args` is a macro that’s used to splice a number of arguments into another term. This is
    /// useful when you want to call a variadic term such as [branch](struct.Client.html#method.branch) with a set of arguments produced at
    /// runtime.
    ///
    /// # Example
    ///
    /// Get Alice and Bob from the table `people`.
    ///
    /// ```
    /// # #![allow(unused_must_use)]
    /// # #[macro_use] extern crate reql;
    /// # use reql::Client;
    /// # fn main() {
    /// # let r = Client::new();
    /// let x = 10;
    /// r.branch(args!(r.expr(x).gt(5), "big", "small"));
    /// # }
    /// ```
    args! => args_impl
);
