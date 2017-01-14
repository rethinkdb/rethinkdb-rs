#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;

pub mod commands;

use ql2::proto::Term;

pub struct Command {
    term: Option<Term>,
    idx: u32,
}

#[allow(non_upper_case_globals)]
pub const r: Command = Command{ term: None, idx: 0 };

pub trait ToArg {
    fn to_arg(&self) -> Vec<Term>;
}
