use super::args::Args;
use crate::{cmd, Func, Query};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

#[derive(
    Debug, Clone, Copy, CommandOptions, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::Filter).with_arg(self).into_arg()
    }
}

impl Arg for Args<(Query, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((arg, opts)) = self;
        arg.arg().with_opts(opts)
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<Options> {
        let Func(arg) = self;
        arg.arg()
    }
}

impl Arg for Args<(Func, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((Func(arg), opts)) = self;
        arg.arg().with_opts(opts)
    }
}
