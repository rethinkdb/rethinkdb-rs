use super::asc::Asc;
use super::desc::Desc;
use crate::cmd;
use crate::proto::{Command, Query};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Index(pub(crate) Command);

#[derive(Serialize)]
struct Inner<'a> {
    index: Query<'a>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        let cmd = Command::from_json(self.into());
        Command::from_json(Inner { index: Query(&cmd) }).into_arg()
    }
}

impl Arg for Asc {
    fn arg(self) -> cmd::Arg<()> {
        let Asc(index) = self;
        Command::from_json(Inner {
            index: Query(&index),
        })
        .into_arg()
    }
}

impl Arg for Desc {
    fn arg(self) -> cmd::Arg<()> {
        let Desc(index) = self;
        Command::from_json(Inner {
            index: Query(&index),
        })
        .into_arg()
    }
}
