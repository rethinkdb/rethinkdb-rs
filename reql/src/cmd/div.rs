use crate::{cmd, Query};
use ql2::term::TermType;
use std::ops::Div;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Div).with_arg(self).into_arg()
    }
}

impl<T> Div<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn div(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_query()
    }
}
