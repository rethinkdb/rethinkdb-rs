use super::asc::Asc;
use super::desc::Desc;
use crate::{cmd, Query};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Index(pub(crate) Query);

#[derive(Serialize)]
struct Inner {
    index: Query,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(Inner {
            index: Query::from_json(self.into()),
        })
        .into_arg()
    }
}

impl Arg for Asc {
    fn arg(self) -> cmd::Arg<()> {
        let Asc(index) = self;
        Query::from_json(Inner { index }).into_arg()
    }
}

impl Arg for Desc {
    fn arg(self) -> cmd::Arg<()> {
        let Desc(index) = self;
        Query::from_json(Inner { index }).into_arg()
    }
}
