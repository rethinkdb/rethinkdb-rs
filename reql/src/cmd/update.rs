use super::args::Args;
use crate::cmd::{self, Durability, ReturnChanges};
use crate::{Func, Query};
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

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        let arg = Query::from_json(self);
        Query::new(TermType::Update).with_arg(arg).into_arg()
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((arg, opts)) = self;
        let arg = Query::from_json(arg);
        arg.arg().with_opts(opts)
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<Options> {
        let Func(func) = self;
        func.arg()
    }
}

impl Arg for Args<(Func, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((Func(func), opts)) = self;
        func.arg().with_opts(opts)
    }
}
