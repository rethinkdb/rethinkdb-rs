use super::args::Args;
use crate::{cmd, Func, Query};
use ql2::term::TermType;
use serde::Serialize;

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
        Query::new(TermType::Map).with_arg(arg).into_arg()
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg()
    }
}

impl<T> Arg for Args<(T, Func)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((sequence, Func(func))) = self;
        sequence.arg().with_arg(func)
    }
}
