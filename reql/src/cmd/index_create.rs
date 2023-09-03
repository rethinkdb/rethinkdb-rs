use super::args::Args;
use crate::{cmd, Command, Func};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::IndexCreate).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        Command::from_json(self.into()).arg()
    }
}

impl<T> Arg for Args<(T, Func)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, Func(func))) = self;
        name.arg().with_arg(func)
    }
}

impl<T, R> Arg for Args<(T, R)>
where
    T: Into<String>,
    R: Into<Command>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, func)) = self;
        name.arg().with_arg(func)
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, opts)) = self;
        name.arg().with_opts(opts)
    }
}

impl<T> Arg for Args<(T, Func, Options)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, Func(func), opts)) = self;
        name.arg().with_arg(func).with_opts(opts)
    }
}

impl<T, R> Arg for Args<(T, R, Options)>
where
    T: Into<String>,
    R: Into<Command>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, func, opts)) = self;
        name.arg().with_arg(func).with_opts(opts)
    }
}
