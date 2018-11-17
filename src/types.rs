//! The ReQL data types

use crate::{
    Request, Result,
    errors::DriverError,
};
use protobuf::ProtobufEnum;
use protobuf::repeated::RepeatedField;
use ql2::proto::{Datum, Datum_AssocPair as DatumPair, Datum_DatumType as DatumType, Term,
                 Term_AssocPair as TermPair, Term_TermType as TermType};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::value::{Value, to_value};

pub trait FromJson {
    fn from_json<T: Serialize>(t: T) -> Result<Term> {
        // Datum
        let mut datum = Datum::new();
        match to_value(t)? {
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
                        let msg = String::from("Value::Number could not be coerced to f64");
                        return Err(DriverError::Other(msg))?;
                    }
                }
            }
            Value::Array(val) => {
                datum.set_field_type(DatumType::R_ARRAY);
                let mut args = Vec::new();
                for a in val.iter() {
                    args.push(Term::from_json(a)?.take_datum());
                }
                let arr = RepeatedField::from_vec(args);
                datum.set_r_array(arr);
            }
            Value::Object(val) => {
                datum.set_field_type(DatumType::R_OBJECT);
                let mut args = Vec::new();
                for (name, arg) in val.into_iter() {
                    let mut obj = DatumPair::new();
                    obj.set_key(name.into());
                    obj.set_val(Term::from_json(arg)?.take_datum());
                    args.push(obj);
                }
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
        Ok(term)
    }
}

impl FromJson for Term {}

trait IsDatum {
    fn is_datum(&self) -> bool;
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Encode {
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

impl Encode for Datum {
    fn encode(&self) -> String {
        match self.get_field_type() {
            DatumType::R_NULL => String::from("null"),
            DatumType::R_BOOL => format!("{}", self.get_r_bool()),
            DatumType::R_NUM => format!("{}", self.get_r_num()),
            DatumType::R_STR => format!("\"{}\"", self.get_r_str()),
            DatumType::R_ARRAY => {
                let mut args = format!("[{},[", TermType::MAKE_ARRAY.value());
                for term in self.get_r_array() {
                    args.push_str(&format!("{},", term.encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("]]");
                args
            }
            DatumType::R_OBJECT => {
                let mut args = String::from("{");
                for term in self.get_r_object() {
                    args.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("}");
                args
            }
            DatumType::R_JSON => {
                unimplemented!();
            }
        }
    }
}

impl<T: DeserializeOwned + Send + std::fmt::Debug> Request<T> {
    pub fn encode(&mut self, data: &Term, encoding_opts: bool) -> String {
        let mut res = Vec::new();
        if !data.is_datum() {
            res.push(format!("[{}", data.get_field_type().value()));
        }
        if data.has_datum() {
            let datum = data.get_datum();
            res.push(datum.encode());
        }
        let terms = data.get_args();
        if !terms.is_empty() {
            let mut args = if data.has_field_type() {
                String::from("[")
            } else {
                String::new()
            };
            for term in terms {
                args.push_str(&format!("{},", self.encode(&term, encoding_opts)));
            }
            args = args.trim_right_matches(",").to_string();
            if data.has_field_type() {
                args.push_str("]");
            }
            res.push(args);
        }
        let opts = data.clone().take_optargs().into_vec();
        if !opts.is_empty() {
            res.push(format!("{}", self.encode_pairs(&opts, encoding_opts)));
        }
        let mut res = res.join(",");
        if !data.is_datum() {
            res.push_str("]");
        }
        res
    }

    fn encode_pairs(&mut self, data: &Vec<TermPair>, encoding_opts: bool) -> String {
        let mut opts = String::from("{");
        for term in data {
            opts.push_str(&format!("\"{}\":{},", term.get_key(), self.encode(term.get_val(), encoding_opts)));
        }
        opts = opts.trim_right_matches(",").to_string();
        opts.push_str("}");
        opts
    }
}
