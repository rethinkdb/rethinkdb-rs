use crate::{cmd, Func, Query};
use ql2::term::TermType;

#[derive(Debug, Clone)]
pub struct Asc(pub(crate) Query);

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for cmd::Arg<()> {
    fn arg(self) -> cmd::Arg<()> {
        self
    }
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Asc).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(self.into()).arg()
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg()
    }
}
