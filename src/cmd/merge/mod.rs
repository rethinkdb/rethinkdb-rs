mod arg;

use crate::cmd::expr::Expr;
use bytes::{BufMut, Bytes, BytesMut};

pub use arg::Arg;

#[derive(Debug, Clone)]
pub struct Merge {
    pub(super) bytes: Bytes,
}

impl Expr {
    pub fn merge<A>(&self, arg: A) -> Merge
    where
        A: Into<Arg>,
    {
        let (header, arg, sep, footer) = ("[35,[", arg.into().bytes, ",", "]]");
        let len = header.len() + self.bytes.len() + arg.len() + sep.len() + footer.len();
        let mut cmd = BytesMut::with_capacity(len);
        cmd.put(header);
        cmd.put(&self.bytes);
        cmd.put(sep);
        cmd.put(arg);
        cmd.put(footer);
        Merge {
            bytes: cmd.freeze(),
        }
    }
}
