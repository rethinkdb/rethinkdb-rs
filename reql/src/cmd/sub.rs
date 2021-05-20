use crate::{cmd, Command};
use ql2::term::TermType;
use std::ops::Sub;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Sub).with_arg(self).into_arg()
    }
}

impl<T> Sub<T> for Command
where
    T: Arg,
{
    type Output = Self;

    fn sub(self, arg: T) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }
}
