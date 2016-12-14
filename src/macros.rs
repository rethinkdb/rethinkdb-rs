macro_rules! Root {
    () => {None as Option<::commands::Command<::ql2::types::Null, ()>>}
}

macro_rules! NoArg {
    () => {None as Option<Vec<::ql2::types::Null>>}
}

macro_rules! var {
    () => {{
        use ::protobuf::repeated::RepeatedField;
        use ::commands::{Client, Command};
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
        Client {
            cmd: Command(From::from(var), None),
            errors: None,
        }
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

macro_rules! opts {
    ($cmd:expr) => {
        match $cmd.1 {
            Some(ref o) => {
                o.clone()
            },
            None => {
                let msg = "Command options are not set. This is a bug in the driver.";
                //crit!(msg; "cmd" => "{:?}", $cmd);
                crit!(msg);
                panic!(msg);
            },
        }
    }
}

#[macro_export]
macro_rules! obj {
    ($( $key:ident: $val:expr ),* $(,)*) => {{
        use ::std::collections::BTreeMap;
        use $crate::Term;

        let mut o = BTreeMap::new();
        $(
            let key = stringify!($key).to_string();
            let val: Term = $val.into();
            o.insert(key, val);
         )*
        let o: Term = o.into();
        o
    }}
}

#[macro_export]
macro_rules! arr {
    ($( $val:expr ),* $(,)*) => {{
        use $crate::Term;

        let v: Vec<Term> = vec![$( $val.into(), )*];
        From::from(v)
    }}
}
