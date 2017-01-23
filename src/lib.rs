//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde;
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

/// The argument that is passed to any ReQL command
pub trait ToArg {
    fn to_arg(&self) -> Term;
}
