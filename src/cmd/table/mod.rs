mod arg;
mod opt;

use {
    crate::{cmd::db::Db, r, ser::to_vec},
    bytes::{BufMut, Bytes, BytesMut},
};

pub use arg::Arg;

#[derive(Debug, Clone)]
pub struct Table {
    pub(super) bytes: Bytes,
}

fn table(prev: Option<&Bytes>, arg: Arg) -> Table {
    let (header, sep, footer) = ("[15,[", ",", "]");
    let args = arg.bytes;
    let opts = to_vec(&arg.opts);
    let opts_len = opts.len();
    let sep_len = sep.len();
    let footer_len = footer.len();
    let prev_len = prev.map(|x| x.len() + sep_len).unwrap_or(0);
    let len = header.len() + prev_len + args.len() + footer_len + opts_len + sep_len + footer_len;
    let mut cmd = BytesMut::with_capacity(len);
    cmd.put(header);
    if let Some(bytes) = prev {
        cmd.put(bytes);
        cmd.put(sep);
    }
    cmd.put(args);
    cmd.put(footer);
    if opts_len > 2 {
        cmd.put(sep);
        cmd.put(opts);
    }
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
        table(None, arg.into())
    }
}

impl Db {
    pub fn table<A>(&self, arg: A) -> Table
    where
        A: Into<Arg>,
    {
        table(Some(&self.bytes), arg.into())
    }
}
