use crate::Query;
use ql2::term::TermType;
use std::ops::BitOr;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::BitOr).with_arg(self)
    }
}

impl<T> BitOr<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn bitor(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
