#[doc(hidden)]
#[macro_export]
macro_rules! var {
    ($idx:expr) => {{
        use $crate::{Client, RepeatedField, Term, Datum, TT, DT};

        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num($idx as f64);
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
    ($f:tt, $( $v:expr ),* $(,)* ) => {{
        use $crate::{Client, RepeatedField, Term, Datum, TT, DT};

        // IDs
        // when closure has no args ids doesn't need to be mutable
        // this lint is ignored (see https://github.com/rust-lang/rust/issues/40491)
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
