use super::args::Args;
use super::index::Index;
use crate::{Func, Query};
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::OrderBy).with_arg(self)
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

impl<T> Arg for Args<(T, Index)>
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        let Args((key, Index(index))) = self;
        Query::from_json(key.into()).into_query().with_arg(index)
    }
}

impl Arg for Func {
    fn into_query(self) -> Query {
        let Func(func) = self;
        func.into_query()
    }
}

impl Arg for Args<(Func, Index)> {
    fn into_query(self) -> Query {
        let Args((Func(func), Index(index))) = self;
        func.into_query().with_arg(index)
    }
}

impl Arg for Index {
    fn into_query(self) -> Query {
        let Index(query) = self;
        query.into_query()
    }
}
