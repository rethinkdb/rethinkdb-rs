use super::args::Args;
use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for cmd::Arg<()> {
    fn arg(self) -> cmd::Arg<()> {
        self
    }
}

impl Arg for () {
    fn arg(self) -> cmd::Arg<()> {
        Query::new(TermType::Uuid).into_arg()
    }
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        ().arg().with_arg(self)
    }
}

impl<T> Arg for Args<T>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args(arg) = self;
        Query::from_json(arg.into()).arg()
    }
}
