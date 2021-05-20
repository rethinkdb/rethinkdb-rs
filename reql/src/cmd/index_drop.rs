use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::IndexDrop).with_arg(self).into_arg()
    }
}

impl Arg for &str {
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}

impl Arg for &String {
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self.as_str()).arg()
    }
}

impl Arg for String {
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}
