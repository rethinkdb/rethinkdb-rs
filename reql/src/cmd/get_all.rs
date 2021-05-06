use super::args::Args;
use super::index::Index;
use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::GetAll).with_arg(self)
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

impl Arg for Args<(&str, Index)> {
    fn into_query(self) -> Query {
        let Args((key, Index(index))) = self;
        key.into_query().with_arg(index)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<[T; N]>
where
    T: Into<String> + Clone,
{
    fn into_query(self) -> Query {
        let Args(arr) = self;
        let mut query = Query::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Query::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Index)>
where
    T: Into<String> + Clone,
{
    fn into_query(self) -> Query {
        let Args((arr, Index(index))) = self;
        let mut query = Query::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Query::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query.with_arg(index)
    }
}
