macro_rules! NoArg {
    () => {None as Option<Vec<::types::Null>>}
}

macro_rules! var {
    ($idx:ident) => {{
        use ::{Client, Command};
        use ::protobuf::repeated::RepeatedField;
        use ::ql2::proto::{
            Term, Datum,
            Term_TermType as TT,
            Datum_DatumType as DT,
        };

        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        *$idx += 1;
        id.set_r_num(*$idx as f64);
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
            idx: *$idx,
            errors: None,
        }
    }}
}

macro_rules! func {
    ($f:expr, $( $v:expr ),* ) => {{
        use ::protobuf::repeated::RepeatedField;
        use ::ql2::proto::{
            Term, Datum,
            Term_TermType as TT,
            Datum_DatumType as DT,
        };

        // IDs
        let mut ids = Vec::new();
        let res = $f(
            $({
                let id = $v;
                let term: Term = id.clone().into();
                for t in term.get_args() {
                    ids.push(t.get_datum().clone());
                }
                id
            },)*
        );
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
        let args = RepeatedField::from_vec(vec![datum, res.into()]);
        func.set_args(args);
        func
    }}
}

macro_rules! err {
    ($e:expr) => {{
        let error = ::errors::Error::from($e);
        Err(error)
    }}
}

#[macro_export]
macro_rules! obj {
    ($( $key:ident: $val:expr ),* $(,)*) => {{
        use ::std::collections::BTreeMap;

        use $crate::{Term, Client};
        use $crate::types::Object;

        let mut o = BTreeMap::new();
        $(
            let key = stringify!($key);
            let val: Term = $val.into();
            o.insert(key, val);
         )*
        let term: Term = o.into();
        let object = Object::from(term);
        Client::from(object)
    }}
}

#[macro_export]
macro_rules! arr {
    ($( $val:expr ),* $(,)*) => {{
        use $crate::{Term, Client};
        use $crate::types::Array;

        let v: Vec<Term> = vec![$( $val.into(), )*];
        let term: Term = v.into();
        let array = Array::from(term);
        Client::from(array)
    }}
}
