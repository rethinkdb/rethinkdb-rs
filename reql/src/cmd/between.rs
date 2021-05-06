use super::args::Args;
use crate::Query;
use ql2::term::TermType;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Between).with_arg(self)
    }
}

impl Arg for Args<(Query, Options<'_>)> {
    fn into_query(self) -> Query {
        let Args((query, opts)) = self;
        query.into_query().with_opts(opts)
    }
}

impl<T> Arg for Args<(T, T)>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let Args((min, max)) = self;
        Query::from_json(min).into_query().with_arg(max.into())
    }
}

impl<T> Arg for Args<(T, T, Options<'_>)>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let Args((min, max, opts)) = self;
        Query::from_json(min)
            .into_query()
            .with_arg(max.into())
            .with_opts(opts)
    }
}
