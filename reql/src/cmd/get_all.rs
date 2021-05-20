use super::args::Args;
use super::index::Index;
use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::GetAll).with_arg(self).into_arg()
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

impl Arg for Args<(&str, Index)> {
    fn arg(self) -> cmd::Arg<()> {
        let Args((key, Index(index))) = self;
        key.arg().with_arg(index)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<[T; N]>
where
    T: Into<String> + Clone,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args(arr) = self;
        let mut query = Command::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Command::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query.into_arg()
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Index)>
where
    T: Into<String> + Clone,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((arr, Index(index))) = self;
        let mut query = Command::new(TermType::GetAll);
        // TODO get rid of the clone in Rust v1.53
        for arg in arr.into_iter().cloned() {
            let arg = Command::from_json(arg.into());
            query = query.with_arg(arg);
        }
        query.with_arg(index).into_arg()
    }
}
