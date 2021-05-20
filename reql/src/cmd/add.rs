use crate::{cmd, Command};
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

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Command::new(TermType::Add).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}

impl<T> Add<T> for Command
where
    T: Arg,
{
    type Output = Self;

    fn add(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_cmd()
    }
}
