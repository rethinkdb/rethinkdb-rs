mod arg;
mod opt;

use {
    crate::{
        cmd::{db::Db, Command},
        r,
    },
    bytes::Bytes,
};

pub use arg::Arg;

#[derive(Debug, Clone)]
pub struct Table {
    pub(super) bytes: Bytes,
}

fn table(prev: &Bytes, arg: Arg) -> Table {
    let Arg { arg, opts } = arg;
    let cmd = Command::new(prev, 15, arg, opts);
    Table { bytes: cmd.into() }
}

impl r {
    pub fn table<A>(&self, arg: A) -> Table
    where
        A: Into<Arg>,
    {
        table(&Bytes::new(), arg.into())
    }
}

impl Db {
    pub fn table<A>(&self, arg: A) -> Table
    where
        A: Into<Arg>,
    {
        table(&self.bytes, arg.into())
    }
}
