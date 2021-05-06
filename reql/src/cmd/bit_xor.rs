use crate::Query;
use ql2::term::TermType;
use std::ops::BitXor;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::BitXor).with_arg(self)
    }
}

impl<T> BitXor<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn bitxor(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
