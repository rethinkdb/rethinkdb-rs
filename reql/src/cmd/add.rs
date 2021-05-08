use crate::{cmd, Query};
use ql2::term::TermType;
use serde::Serialize;
use std::ops::Add;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for cmd::Arg<()> {
    fn arg(self) -> cmd::Arg<()> {
        self
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let arg = Query::from_json(self);
        Query::new(TermType::Add).with_arg(arg).into_arg()
    }
}

impl<T> Add<T> for Query
where
    T: Arg,
{
    type Output = Self;

    fn add(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_query()
    }
}
