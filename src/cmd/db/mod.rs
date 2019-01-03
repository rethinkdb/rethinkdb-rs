mod arg;

use crate::r;
use bytes::{BufMut, Bytes, BytesMut};

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Db {
    pub(super) bytes: Bytes,
}

impl r {
    pub fn db<A>(self, arg: A) -> Db
    where
        A: Into<Arg>,
    {
        let (header, arg, footer) = ("[14,[", arg.into().bytes, "]]");
        let len = header.len() + arg.len() + footer.len();
        let mut cmd = BytesMut::with_capacity(len);
        cmd.put(header);
        cmd.put(arg);
        cmd.put(footer);
        Db {
            bytes: cmd.freeze(),
        }
    }
}
