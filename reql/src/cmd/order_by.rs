use super::args::Args;
use super::index::Index;
use crate::{cmd, Command, Func};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::OrderBy).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self.into()).arg()
    }
}

impl<T> Arg for Args<(T, Index)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((key, Index(index))) = self;
        Command::from_json(key.into()).arg().with_arg(index)
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg()
    }
}

impl Arg for Args<(Func, Index)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((Func(func), Index(index))) = self;
        func.arg().with_arg(index)
    }
}

impl Arg for Index {
    fn arg(self) -> cmd::Arg<()> {
        let Index(query) = self;
        query.arg()
    }
}
