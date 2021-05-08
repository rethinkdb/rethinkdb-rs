use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Nth).with_arg(self).into_arg()
    }
}

impl Arg for isize {
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(self).arg()
    }
}
