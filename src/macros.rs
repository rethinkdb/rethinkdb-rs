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
