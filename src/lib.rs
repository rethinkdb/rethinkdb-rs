//! A native RethinkDB driver written in Rust

// `expr` macro recurses deeply

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
pub use ql2::proto::{Term, Term_AssocPair as TermPair};

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
pub trait IntoArg {
    fn into_arg(&self) -> Vec<Term>;
}

impl Command {
    pub fn new(term: Term) -> Command {
        Command {
            term: term,
        }
    }
}
