use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Branch).with_arg(self)
    }
}

impl<T> Arg for (T, Query)
where
    T: Arg,
{
    fn into_query(self) -> Query {
        let (left, right) = self;
        left.into_query().with_arg(right)
    }
}
