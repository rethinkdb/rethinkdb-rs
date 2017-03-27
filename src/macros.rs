macro_rules! with_args {
    ( $cmd:ident, $args:ident ) => {{
        let mut tmp_args = $args.term;
        if tmp_args.has_field_type() { // did not come from the args macro
            $cmd.term.mut_args().push(tmp_args);
        } else { // came from the args macro
            for arg in tmp_args.take_args().into_vec() {
                $cmd.term.mut_args().push(arg);
            }
            for pair in tmp_args.take_optargs().into_vec() {
                $cmd.term.mut_optargs().push(pair);
            }
        }
    }}
}

macro_rules! bail_result {
    ($qry:ident) => {
        if let ::QueryError::Some(ref qry) = $qry.error {
            unimplemented!();
            //return Err(qry.clone().1)?;
        }
    }
}

macro_rules! bail_client {
    ($qry:ident, $cli:ident) => {
        if let ::QueryError::Some(_) = $cli.error {
            return $cli.clone();
        }
        if let ::QueryError::Some(ref qry) = $qry.error {
            let mut cmd = $cli.clone();
            cmd.error = ::QueryError::Some(qry.clone());
            return cmd;
        }
    }
}

/// Splice an array of arguments into another term
///
/// `args` is a macro that’s used to splice a number of arguments into another term. This is
/// useful when you want to call a variadic term such as [branch](commands/trait.Branch.html) with a set of arguments produced at
/// runtime.
///
/// *Note* that args evaluates all its arguments before passing them into the parent term, even if
/// the parent term otherwise allows lazy evaluation.
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
proc_macro_expr_decl!(args! => args_impl);
