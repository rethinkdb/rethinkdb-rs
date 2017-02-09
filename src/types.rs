//! The ReQL data types

use ql2::proto::{Term, Datum,
    Term_TermType as TermType,
    Term_AssocPair as TermPair,
    Datum_DatumType as DatumType,
    Datum_AssocPair as DatumPair};
use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;

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

trait IsDatum {
    fn is_datum(&self) -> bool;
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

trait Encode {
    fn encode(&self) -> String;
}

impl IsDatum for Term {
    fn is_datum(&self) -> bool {
        self.get_field_type() == TermType::DATUM
    }
}

impl IsEmpty for Term {
    fn is_empty(&self) -> bool {
        *self == Term::new()
    }
}

impl Encode for Vec<TermPair> {
    fn encode(&self) -> String {
        let mut opts = String::from("{");
        for term in self {
            opts.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
        }
        opts = opts.trim_right_matches(",").to_string();
        opts.push_str("}");
        opts
    }
}

impl Encode for Datum {
    fn encode(&self) -> String {
        match self.get_field_type() {
            DatumType::R_NULL => {
                String::from("null")
            },
            DatumType::R_BOOL => {
                format!("{}", self.get_r_bool())
            },
            DatumType::R_NUM => {
                format!("{}", self.get_r_num())
            },
            DatumType::R_STR => {
                format!("\"{}\"", self.get_r_str())
            },
            DatumType::R_ARRAY => {
                let mut args = format!("[{},[", TermType::MAKE_ARRAY.value());
                for term in self.get_r_array() {
                    args.push_str(&format!("{},", term.encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("]]");
                args
            },
            DatumType::R_OBJECT => {
                let mut args = String::from("{");
                for term in self.get_r_object() {
                    args.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("}");
                args
            },
            DatumType::R_JSON => {
                unimplemented!();
            },
        }
    }
}

impl Encode for Term {
    fn encode(&self) -> String {
        let mut res = Vec::new();
        if !self.is_datum() {
            res.push(format!("[{}", self.get_field_type().value()));
        }
        if self.has_datum() {
            let datum = self.get_datum();
            res.push(datum.encode());
        }
        let terms = self.get_args();
        if !terms.is_empty() {
            let mut args = String::from("[");
            for term in terms {
                args.push_str(&format!("{},", term.encode()));
            }
            args = args.trim_right_matches(",").to_string();
            args.push_str("]");
            res.push(args);
        }
        let opts = self.clone().take_optargs().into_vec();
        if !opts.is_empty() {
            res.push(format!("{}", opts.encode()));
        }
        let mut res = res.join(",");
        if !self.is_datum() {
            res.push_str("]");
        }
        res
    }
}
