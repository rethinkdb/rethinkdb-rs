use super::args::Args;
use crate::{Func, Query};
use ql2::term::TermType;
use serde_json::Value;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Funcall).with_arg(self)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([Query; N], Query)> {
    fn into_query(self) -> Query {
        let Args((arr, expr)) = self;
        let mut query = expr.into_query();
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            query = query.with_arg(arg);
        }
        query
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Query)>
where
    T: Into<Value> + Clone,
{
    fn into_query(self) -> Query {
        let Args((arr, expr)) = self;
        let mut query = expr.into_query();
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Query::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query
    }
}

impl Arg for Args<(Query, Func)> {
    fn into_query(self) -> Query {
        let Args((query, Func(func))) = self;
        func.into_query().with_arg(query)
    }
}

impl Arg for Func {
    fn into_query(self) -> Query {
        let Func(func) = self;
        func.into_query()
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([Query; N], Func)> {
    fn into_query(self) -> Query {
        let Args((arr, Func(func))) = self;
        let mut query = func.into_query();
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            query = query.with_arg(arg);
        }
        query
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Func)>
where
    T: Into<Value> + Clone,
{
    fn into_query(self) -> Query {
        let Args((arr, Func(func))) = self;
        let mut query = func.into_query();
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Query::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query
    }
}
