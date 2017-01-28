//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_error;
extern crate r2d2;
#[macro_use]
extern crate serde_derive;
extern crate scram;
extern crate byteorder;
extern crate bufstream;
extern crate uuid;

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;
mod types;
mod args;
mod conn;
pub mod commands;
pub mod errors;

#[doc(hidden)]
pub use ql2::proto::Term;

use protobuf::ProtobufEnum;
use ql2::proto::{Datum,
    Term_TermType as TT,
    Term_AssocPair as TA,
    Datum_DatumType as DT,
};

pub type Result<T> = ::std::result::Result<T, errors::Error>;

/// The argument that is passed to any ReQL command
pub trait ToArg {
    fn to_arg(&self) -> Term;
}

pub trait IsDatum {
    fn is_datum(&self) -> bool;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Encode {
    fn encode(&self) -> String;
}

impl IsDatum for Term {
    fn is_datum(&self) -> bool {
        self.get_field_type() == TT::DATUM
    }
}

impl IsEmpty for Term {
    fn is_empty(&self) -> bool {
        *self == Term::new()
    }
}

impl Encode for Vec<TA> {
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
            DT::R_NULL => {
                String::from("null")
            },
            DT::R_BOOL => {
                format!("{}", self.get_r_bool())
            },
            DT::R_NUM => {
                format!("{}", self.get_r_num())
            },
            DT::R_STR => {
                format!("\"{}\"", self.get_r_str())
            },
            DT::R_ARRAY => {
                let mut args = format!("[{},[", TT::MAKE_ARRAY.value());
                for term in self.get_r_array() {
                    args.push_str(&format!("{},", term.encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("]]");
                args
            },
            DT::R_OBJECT => {
                let mut args = String::from("{");
                for term in self.get_r_object() {
                    args.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("}");
                args
            },
            DT::R_JSON => {
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
