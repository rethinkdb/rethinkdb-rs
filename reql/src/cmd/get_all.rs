use super::args::Args;
use crate::{cmd, Command};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    pub index: Option<Cow<'static, str>>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::GetAll).with_arg(self).into_arg()
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

impl Arg for Args<(&str, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((key, opts)) = self;
        key.arg().with_opts(opts)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<[T; N]>
where
    T: Into<String> + Clone,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args(arr) = self;
        let mut query = Command::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Command::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query.into_arg()
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Options)>
where
    T: Into<String> + Clone,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((arr, opts)) = self;
        let mut query = Command::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Command::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query.with_opts(opts).into_arg()
    }
}
