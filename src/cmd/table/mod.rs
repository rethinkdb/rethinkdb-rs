mod arg;

use crate::{cmd::db::Db, r};
use bytes::{BufMut, Bytes, BytesMut};

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Table {
    pub(super) bytes: Bytes,
}

fn table(prev: Option<&Bytes>, arg: Bytes) -> Table {
    let (header, sep, footer) = ("[15,[", ",", "],{}]");
    let len =
        header.len() + prev.map(|x| x.len() + sep.len()).unwrap_or(0) + arg.len() + footer.len();
    let mut cmd = BytesMut::with_capacity(len);
    cmd.put(header);
    if let Some(bytes) = prev {
        cmd.put(bytes);
        cmd.put(sep);
    }
    cmd.put(arg);
    cmd.put(footer);
    Table {
        bytes: cmd.freeze(),
    }
}

impl r {
    pub fn table<A>(&self, arg: A) -> Table
    where
        A: Into<Arg>,
    {
        table(None, arg.into().bytes)
    }
}

impl Db {
    pub fn table<A>(&self, arg: A) -> Table
    where
        A: Into<Arg>,
    {
        table(Some(&self.bytes), arg.into().bytes)
    }
}
