use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for cmd::Arg<()> {
    fn arg(self) -> cmd::Arg<()> {
        self
    }
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::And).with_arg(self).into_arg()
    }
}
