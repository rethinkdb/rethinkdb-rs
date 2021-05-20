use super::args::Args;
use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::DeleteAt).with_arg(self).into_arg()
    }
}

impl Arg for i64 {
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}

impl Arg for Args<[i64; 2]> {
    fn arg(self) -> cmd::Arg<()> {
        let Args([offset, end_offset]) = self;
        Command::from_json(offset)
            .arg()
            .with_arg(Command::from_json(end_offset))
    }
}
