use super::args::Args;
use crate::cmd::{Durability, ReturnChanges};
use crate::{cmd, Command};
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

impl Arg for Command {
    fn arg(self) -> cmd::Arg<Options> {
        Command::new(TermType::Insert).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        let arg = Command::from_json(self);
        Command::new(TermType::Insert).with_arg(arg).into_arg()
    }
}

impl Arg for Args<(Command, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((val, options)) = self;
        val.arg().with_opts(options)
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((val, options)) = self;
        Command::from_json(val).arg().with_opts(options)
    }
}
