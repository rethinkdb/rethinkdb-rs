use super::args::Args;
use crate::{Func, Query};
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Filter).with_arg(self)
    }
}

impl Arg for Args<(Query, Options)> {
    fn into_query(self) -> Query {
        let Args((arg, opts)) = self;
        arg.into_query().with_opts(opts)
    }
}

impl Arg for Func {
    fn into_query(self) -> Query {
        let Func(arg) = self;
        arg.into_query()
    }
}

impl Arg for Args<(Func, Options)> {
    fn into_query(self) -> Query {
        let Args((Func(arg), opts)) = self;
        arg.into_query().with_opts(opts)
    }
}
