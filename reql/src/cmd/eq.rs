use crate::{cmd, Query};
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

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let arg = Query::from_json(self);
        Query::new(TermType::Eq).with_arg(arg).into_arg()
    }
}
