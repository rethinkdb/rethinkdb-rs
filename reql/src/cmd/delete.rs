use crate::cmd::{self, Durability, ReturnChanges};
use crate::Command;
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

// TODO finish this struct
#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for cmd::Arg<Options> {
    fn arg(self) -> cmd::Arg<Options> {
        self
    }
}

impl Arg for () {
    fn arg(self) -> cmd::Arg<Options> {
        Command::new(TermType::Delete).into_arg()
    }
}

impl Arg for Options {
    fn arg(self) -> cmd::Arg<Self> {
        ().arg().with_opts(self)
    }
}
