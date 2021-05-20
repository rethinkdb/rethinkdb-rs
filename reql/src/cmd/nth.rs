use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Nth).with_arg(self).into_arg()
    }
}

impl Arg for isize {
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}
