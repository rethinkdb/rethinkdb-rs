use crate::Query;
use ql2::term::TermType;
use std::ops::Sub;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Sub).with_arg(self)
    }
}

impl<T> Sub<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn sub(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
