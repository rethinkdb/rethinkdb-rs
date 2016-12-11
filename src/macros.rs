macro_rules! Root {
    () => {None as Option<&Command<::ql2::types::Null, ()>>}
}

macro_rules! NoArg {
    () => {None as Option<Vec<::ql2::types::Null>>}
}

macro_rules! var {
    () => {{
        use ::protobuf::repeated::RepeatedField;
        use ::commands::Command;
        use ::ql2::proto::{
            Term, Datum,
            Term_TermType as TT,
            Datum_DatumType as DT,
        };

        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num(1.0);
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(id);
        // VAR
        let mut var = Term::new();
        var.set_field_type(TT::VAR);
        let args = RepeatedField::from_vec(vec![datum]);
        var.set_args(args);
        Command(From::from(var), None)
    }}
}

macro_rules! func {
    ($res:expr) => {{
        use ::protobuf::repeated::RepeatedField;
        use ::ql2::proto::{
            Term, Datum,
            Term_TermType as TT,
            Datum_DatumType as DT,
        };

        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num(1.0);
        // ARRAY
        let mut array = Datum::new();
        array.set_field_type(DT::R_ARRAY);
        let args = RepeatedField::from_vec(vec![id]);
        array.set_r_array(args);
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(array);
        // FUNC
        let mut func = Term::new();
        func.set_field_type(TT::FUNC);
        let args = RepeatedField::from_vec(vec![datum, $res]);
        func.set_args(args);
        func
    }}
}

macro_rules! err {
    ($e:expr) => {{
        let error = ::ql2::errors::Error::from($e);
        Err(error)
    }}
}

macro_rules! set_opt {
    ($opts:ident, $func:ident($arg:ident)) => {
        match $opts.1 {
            Some(ref mut opts) => {
                opts.$func = $arg;
            }
            None => {
                $opts.1 = Some(Default::default());
                if let Some(ref mut opts) = $opts.1 {
                    opts.$func = $arg;
                }
            }
        }
    }
}
