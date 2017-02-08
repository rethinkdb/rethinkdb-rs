//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate derive_error;
#[cfg(feature = "with_io")]
extern crate reql_io;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod types;
pub mod commands;
pub mod errors;

#[doc(hidden)]
pub use ql2::proto::Term;

use std::net::SocketAddr;

use errors::Error;

#[cfg(feature = "with_io")]
use reql_io::r2d2;

use slog::Logger;
use protobuf::ProtobufEnum;
use ql2::proto::{Datum,
    Term_TermType as TT,
    Term_AssocPair as TA,
    Datum_DatumType as DT,
};

/// The result of any ReQL command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `ToArg::to_arg`
///
/// It's not meant to be used directly.
pub struct Arg {
    string: String,
    term: Term,
}

/// The response returned by the `run` command
pub struct Response<T>(T);

/// The ReQL connection returned by the `connect` command
///
/// Internally this is actually a connection pool.
pub struct Connection;

/// The configuration data for the `connect` command
#[derive(Debug)]
pub struct Config(Vec<InnerConfig>);

#[derive(Debug)]
struct InnerConfig {
    pool: r2d2::Config<Connection, Error>,
    addresses: Vec<SocketAddr>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: &'static str,
}

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Command {
    term: Term,
    query: String,
    logger: Logger,
}

impl Arg {
    #[doc(hidden)]
    pub fn term(self) -> Term {
        self.term
    }
}

/// The return type of the `args!()` macro
#[derive(Debug, Clone)]
pub struct Args {
    term: Term,
    string: String,
}

/// The argument that is passed to any ReQL command
pub trait ToArg {
    fn to_arg(&self) -> Arg;
}

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
