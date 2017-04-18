//! The ReQL data types

use std::result;
use std::ops::Deref;

use {Result, DateTime, chrono};
use errors::DriverError;
use ql2::proto::{Term, Datum, Term_TermType as TermType, Term_AssocPair as TermPair,
                 Datum_DatumType as DatumType, Datum_AssocPair as DatumPair};
use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub trait FromJson {
    fn from_json<T: ToJson>(t: T) -> Result<Term> {
        // Datum
        let mut datum = Datum::new();
        match t.to_json()? {
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
            let mut args = if self.has_field_type() {
                String::from("[")
            } else {
                String::new()
            };
            for term in terms {
                args.push_str(&format!("{},", term.encode()));
            }
            args = args.trim_right_matches(",").to_string();
            if self.has_field_type() {
                args.push_str("]");
            }
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
        // replace closure placeholders
        let parts: Vec<&str> = res.split("\"VARID-C1058970-A4C6-47A8-AD25-1113EA72F84E\"").collect();
        let mut res = String::new();
        for (i, part) in parts.into_iter().enumerate() {
            if i != 0 {
                res.push_str(&i.to_string());
            }
            res.push_str(&part);
        }
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    #[serde(rename = "$reql_type$")]
    reql_type: String,
    epoch_time: f64,
    timezone: String,
}

impl Deserialize for DateTime {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer
    {
        let time = Time::deserialize(deserializer)?;
        let secs = time.epoch_time.trunc() as i64;
        let nsecs = time.epoch_time.fract().abs() as u32;
        let naive = chrono::NaiveDateTime::from_timestamp(secs, nsecs);
        let dt = chrono::DateTime::<chrono::UTC>::from_utc(naive, chrono::UTC);
        Ok(DateTime(dt))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, _serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        unimplemented!();
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::UTC>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
