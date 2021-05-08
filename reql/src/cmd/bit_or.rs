use crate::{cmd, Query};
use ql2::term::TermType;
use std::ops::BitOr;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::BitOr).with_arg(self).into_arg()
    }
}

impl<T> BitOr<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn bitor(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_query()
    }
}
