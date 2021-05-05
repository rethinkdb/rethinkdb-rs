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
        Query::new(TermType::IndexCreate).with_arg(self)
    }
}

impl Arg for String {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for &str {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for &String {
    fn into_query(self) -> Query {
        self.as_str().into_query()
    }
}

impl<T> Arg for (T, Func)
where
    T: Arg,
{
    fn into_query(self) -> Query {
        let (name, func) = self;
        name.into_query().with_arg(func)
    }
}

impl<T> Arg for (T, Options)
where
    T: Arg,
{
    fn into_query(self) -> Query {
        let (arg, opts) = self;
        arg.into_query().with_opts(opts)
    }
}
