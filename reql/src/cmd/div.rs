use crate::Query;
use ql2::term::TermType;
use std::ops::Div;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Div).with_arg(self)
    }
}

impl<T> Div<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn div(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
