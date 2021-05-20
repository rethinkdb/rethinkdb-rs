use super::args::Args;
use crate::{cmd, Command};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

#[derive(
    Debug, Clone, Copy, CommandOptions, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[non_exhaustive]
pub struct Options<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    Open,
    Closed,
}

pub trait Arg<'a> {
    fn arg(self) -> cmd::Arg<Options<'a>>;
}

impl<'a> Arg<'a> for Command {
    fn arg(self) -> cmd::Arg<Options<'a>> {
        Self::new(TermType::Between).with_arg(self).into_arg()
    }
}

impl<'a> Arg<'a> for Args<(Command, Options<'a>)> {
    fn arg(self) -> cmd::Arg<Options<'a>> {
        let Args((query, opts)) = self;
        query.arg().with_opts(opts)
    }
}

impl<'a, T> Arg<'a> for Args<(T, T)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options<'a>> {
        let Args((min, max)) = self;
        let max = Command::from_json(max);
        Command::from_json(min).arg().with_arg(max)
    }
}

impl<'a, T> Arg<'a> for Args<(T, T, Options<'a>)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options<'a>> {
        let Args((min, max, opts)) = self;
        let max = Command::from_json(max);
        Command::from_json(min).arg().with_arg(max).with_opts(opts)
    }
}
