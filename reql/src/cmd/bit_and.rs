use crate::Query;
use ql2::term::TermType;
use std::ops::BitAnd;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::BitAnd).with_arg(self)
    }
}

impl<T> BitAnd<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn bitand(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
