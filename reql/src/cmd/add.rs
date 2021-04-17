use crate::Query;
use ql2::term::TermType;
use std::ops::Add;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Add).with_arg(self)
    }
}

impl<T> Add<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn add(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
