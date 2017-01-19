//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;

#[cfg(test)]
mod tests;

macro_rules! commands {
    ($($cmd:ident),* $(,)*) => {
        $(
            mod $cmd;
            pub use self::$cmd::*;
        )*
    }
}

macro_rules! command {
    ( $(#[$attr:meta])* ) => {
        #[derive(Command)]
        $(#[$attr])*
        struct _Dummy;
    }
}

pub mod commands;
mod types;
mod args;

use ql2::proto::Term;

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Command {
    term: Option<Term>,
    idx: u32,
}

/// The top-level ReQL namespace
#[allow(non_upper_case_globals)]
pub const r: Command = Command{ term: None, idx: 0 };

/// The argument that is passed to any ReQL command
pub trait IntoArg {
    fn into_arg(self) -> Vec<Term>;
}
