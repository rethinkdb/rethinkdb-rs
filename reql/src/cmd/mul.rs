use crate::{cmd, Query};
use ql2::term::TermType;
use std::ops::Mul;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Mul).with_arg(self).into_arg()
    }
}

impl<T> Mul<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn mul(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_query()
    }
}
