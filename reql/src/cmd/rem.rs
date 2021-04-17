use crate::Query;
use ql2::term::TermType;
use std::ops::Rem;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Mod).with_arg(self)
    }
}

impl<T> Rem<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn rem(self, arg: T) -> Self {
        arg.into_query().with_parent(self)
    }
}
