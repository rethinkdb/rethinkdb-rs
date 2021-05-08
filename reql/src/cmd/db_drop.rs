use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::DbDrop).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(self.into()).arg()
    }
}
