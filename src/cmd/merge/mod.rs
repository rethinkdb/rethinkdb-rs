mod arg;

use crate::{
    cmd::{
        connect::Connection,
        run::{run, Opts},
    },
    Result,
};
use bytes::{BufMut, Bytes, BytesMut};
use serde::de::DeserializeOwned;

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Merge(Bytes);

pub(crate) fn merge<A>(prev: &Bytes, arg: A) -> Merge
where
    A: Into<Arg>,
{
    let arg = arg.into().0;
    let mut cmd = BytesMut::with_capacity(1024);
    cmd.put("[35,[");
    cmd.put(prev.as_ref());
    cmd.put(",");
    cmd.put(arg.as_ref());
    cmd.put("],{}]");
    Merge(cmd.freeze())
}

impl Merge {
    pub async fn run<O, T>(self, conn: &Connection, opts: O) -> Result<T>
    where
        O: Into<Option<Opts>> + 'static,
        T: DeserializeOwned,
    {
        await!(run(conn, self.0, opts.into()))
    }
}
