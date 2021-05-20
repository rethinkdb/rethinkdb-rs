use super::args::Args;
use crate::{cmd, Command, Func};
use ql2::term::TermType;
use serde::Serialize;

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
        Command::new(TermType::Map).with_arg(self).into_arg()
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg()
    }
}

impl Arg for Args<(Command, Func)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((sequence, Func(func))) = self;
        sequence.arg().with_arg(func)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([Command; N], Func)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((sequence, Func(func))) = self;
        let mut cmd = cmd::Arg::new();
        if N == 0 {
            func.arg()
        } else {
            for (i, arg) in sequence.into_iter().cloned().enumerate() {
                if i == 0 {
                    cmd = arg.arg();
                } else {
                    cmd = cmd.with_arg(arg);
                }
            }
            cmd.with_arg(func)
        }
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Func)>
where
    T: Serialize + Clone,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((sequence, Func(func))) = self;
        let mut cmd = cmd::Arg::new();
        if N == 0 {
            func.arg()
        } else {
            for (i, arg) in sequence.into_iter().cloned().enumerate() {
                let arg = Command::from_json(arg);
                if i == 0 {
                    cmd = arg.arg();
                } else {
                    cmd = cmd.with_arg(arg);
                }
            }
            cmd.with_arg(func)
        }
    }
}
