use crate::Query;
use ql2::term::TermType;
use std::ops::Mul;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Mul).with_arg(self)
    }
}

impl<T> Mul<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn mul(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
