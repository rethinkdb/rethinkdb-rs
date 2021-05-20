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

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Append).with_arg(self).into_arg()
    }
}
