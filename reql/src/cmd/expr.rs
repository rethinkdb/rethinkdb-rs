use crate::{cmd, Query};
use serde::Serialize;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(self).into_arg()
    }
}
