use super::args::Args;
use crate::{Func, Query};
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::IndexCreate).with_arg(self)
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        Query::from_json(self.into()).into_query()
    }
}

impl<T> Arg for Args<(T, Func)>
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        let Args((name, func)) = self;
        name.into_query().with_arg(func)
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        let Args((name, opts)) = self;
        name.into_query().with_opts(opts)
    }
}

impl<T> Arg for Args<(T, Func, Options)>
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        let Args((name, func, opts)) = self;
        name.into_query().with_arg(func).with_opts(opts)
    }
}
