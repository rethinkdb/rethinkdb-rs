use crate::{cmd, Query};
use ql2::term::TermType;
use std::ops::Sub;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Sub).with_arg(self).into_arg()
    }
}

impl<T> Sub<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn sub(self, arg: T) -> Self {
        arg.arg().into_query().with_parent(self)
    }
}
