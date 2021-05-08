use crate::{cmd, Query};
use ql2::term::TermType;
use serde::Serialize;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let arg = Query::from_json(self);
        Query::new(TermType::Get).with_arg(arg).into_arg()
    }
}
