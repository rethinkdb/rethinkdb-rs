//! The ReQL data types

use ql2::proto::{Term, Datum,
    Term_TermType as TermType,
    Datum_DatumType as DatumType,
    Datum_AssocPair as DatumPair};
use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;
use std::ops::Add;
use commands::Add as AddCmd;
use {Command, ToArg};

impl<'a> Add for &'a ToArg {
    type Output = Command;

    fn add(self, other: &'a ToArg) -> Command {
        expr!(self).add((*other).to_arg())
    }
}

pub trait FromJson {
    fn from_json<T: ToJson>(t: T) -> Term {
        // Datum
        let mut datum = Datum::new();
        match t.to_json() {
            Value::String(val) => {
                datum.set_field_type(DatumType::R_STR);
                datum.set_r_str(val);
            }
            Value::Bool(val) => {
                datum.set_field_type(DatumType::R_BOOL);
                datum.set_r_bool(val);
            }
            Value::I64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val as f64);
            }
            Value::U64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val as f64);
            }
            Value::F64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val);
            }
            Value::Array(val) => {
                datum.set_field_type(DatumType::R_ARRAY);
                let args: Vec<Datum> = val.iter()
                    .map(|a| Term::from_json(a).take_datum())
                    .collect();
                let arr = RepeatedField::from_vec(args);
                datum.set_r_array(arr);
            }
            Value::Object(val) => {
                datum.set_field_type(DatumType::R_OBJECT);
                let args: Vec<DatumPair> = val.into_iter()
                    .map(|(name, arg)| {
                        let mut obj = DatumPair::new();
                        obj.set_key(name.into());
                        obj.set_val(Term::from_json(arg).take_datum());
                        obj
                    })
                    .collect();
                let obj = RepeatedField::from_vec(args);
                datum.set_r_object(obj);
            }
            Value::Null => {
                datum.set_field_type(DatumType::R_NULL);
            }
        }
        // Term
        let mut term = Term::new();
        term.set_field_type(TermType::DATUM);
        term.set_datum(datum);
        term
    }
}

impl FromJson for Term { }
