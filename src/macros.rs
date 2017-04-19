#[doc(hidden)]
#[macro_export]
macro_rules! var {
    () => {{
        use $crate::{Client, RepeatedField, Term, Datum, TT, DT};

        // ID
        let mut id = Datum::new();
        //id.set_field_type(DT::R_NUM);
        //id.set_r_num(idx as f64);
        id.set_field_type(DT::R_STR);
        id.set_r_str("VARID-C1058970-A4C6-47A8-AD25-1113EA72F84E".into());
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(id);
        // VAR
        let mut var = Term::new();
        var.set_field_type(TT::VAR);
        let args = RepeatedField::from_vec(vec![datum]);
        var.set_args(args);
        let mut client = Client::new();
        client.set_term(Ok(var));
        client
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! func {
    ($f:tt, $( $v:expr ),* ) => {{
        use $crate::{Client, RepeatedField, Term, Datum, TT, DT};

        // IDs
        #[allow(unused_mut)]
        let mut ids = Vec::new();
        let res: Client = $f(
            $({
                let id = $v;
                for t in id.term().unwrap().get_args() {
                    ids.push(t.get_datum().clone());
                }
                id
            },)*
        );
        let mut closure = Client::new();
        match res.term() {
            Ok(res) => {
                // ARRAY
                let mut array = Datum::new();
                array.set_field_type(DT::R_ARRAY);
                let args = RepeatedField::from_vec(ids);
                array.set_r_array(args);
                // DATUM
                let mut datum = Term::new();
                datum.set_field_type(TT::DATUM);
                datum.set_datum(array);
                // FUNC
                let mut func = Term::new();
                func.set_field_type(TT::FUNC);
                let args = RepeatedField::from_vec(vec![datum, res.clone()]);
                func.set_args(args);
                closure.set_term(Ok(func));
            }
            Err(error) => {
                closure.set_term(Err(error));
            }
        }
        closure
    }}
}

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
