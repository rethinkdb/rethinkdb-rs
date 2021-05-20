use super::args::Args;
use crate::{cmd, Command};
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
        Command::new(TermType::Uuid).into_arg()
    }
}

impl Arg for Command {
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
        Command::from_json(arg.into()).arg()
    }
}
