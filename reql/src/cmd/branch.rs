use super::args::Args;
use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Branch).with_arg(self).into_arg()
    }
}

impl Arg for Args<(Command, Command, Command)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((test, true_action, false_action)) = self;
        test.arg().with_arg(true_action).with_arg(false_action)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([(Command, Command); N], Command)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((arr, false_action)) = self;
        let mut query = Command::new(TermType::Branch);
        // TODO remove the clone in Rust v1.53
        for (test, true_action) in arr.into_iter().cloned() {
            query = query.with_arg(test).with_arg(true_action);
        }
        query.with_arg(false_action).into_arg()
    }
}
