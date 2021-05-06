use super::args::Args;
use crate::{Func, Query};
use ql2::term::TermType;
use serde::Serialize;
use serde_json::Value;

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Options {}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Update).with_arg(self)
    }
}

impl Arg for Args<(Query, Options)> {
    fn into_query(self) -> Query {
        let Args((arg, opts)) = self;
        arg.into_query().with_opts(opts)
    }
}

impl<T> Arg for T
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        Query::from_json(self.into()).into_query()
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let Args((arg, opts)) = self;
        arg.into_query().with_opts(opts)
    }
}

impl Arg for Func {
    fn into_query(self) -> Query {
        let Func(func) = self;
        func.into_query()
    }
}

impl Arg for Args<(Func, Options)> {
    fn into_query(self) -> Query {
        let Args((Func(func), opts)) = self;
        func.into_query().with_opts(opts)
    }
}
