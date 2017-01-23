//! A native RethinkDB driver written in Rust

// Currently can't set these within lazy_static
// These are for `r`
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;
mod types;
mod args;
pub mod commands;

#[doc(hidden)]
pub use ql2::proto::Term;

use ql2::proto::Term_AssocPair as TermPair;

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Command {
    term: Term,
}

lazy_static! {
    /// The top-level ReQL namespace
    pub static ref r: Command = Command {
        term: Term::new(),
    };
}

/// The argument that is passed to any ReQL command
pub trait ToArg {
    fn to_arg(&self) -> Term;
}

impl Command {
    pub fn new(term: Term) -> Command {
        Command {
            term: term,
        }
    }

    #[doc(hidden)]
    pub fn create_term_pair<T: ToArg>(key: &str, val: T) -> TermPair {
        let mut temp = Term::new();
        temp.mut_args().push(val.to_arg());
        let mut temp_pair = TermPair::new();
        temp_pair.set_key(key.into());
        temp_pair.set_val(temp);
        temp_pair
    }
}
