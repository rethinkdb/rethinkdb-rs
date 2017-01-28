//! The ReQL data types

use ql2::proto::{Term, Datum,
    Term_TermType as TermType,
    Datum_DatumType as DatumType,
    Datum_AssocPair as DatumPair};
use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;

pub trait FromJson {
    fn from_json<T: ToJson>(t: T) -> Term {
        // Datum
        let mut datum = Datum::new();
        match t.to_json() {
            Ok(val) => {
                match val {
                    Value::String(val) => {
                        datum.set_field_type(DatumType::R_STR);
                        datum.set_r_str(val);
                    }
                    Value::Bool(val) => {
                        datum.set_field_type(DatumType::R_BOOL);
                        datum.set_r_bool(val);
                    }
                    Value::Number(val) => {
                        match val.as_f64() {
                            Some(val) => {
                                datum.set_field_type(DatumType::R_NUM);
                                datum.set_r_num(val);
                            }
                            None => {
                                // @TODO: handle this at compile time
                                unreachable!();
                            }
                        }
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
            }
            Err(_) => {
                // @TODO handle this at compile time
                unreachable!();
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
