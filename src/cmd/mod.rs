//! ReQL command reference

pub mod connect;
pub mod db;
pub mod expr;
pub mod merge;
pub mod run;
pub mod table;

use bytes::{BufMut, Bytes, BytesMut};

#[doc(hidden)]
macro make_builder() {
    /// Start building the options
    pub fn builder() -> Self {
        Default::default()
    }

    /// Finalise the options
    pub fn build(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
struct Command<'a> {
    id: u16,
    prev: &'a [u8],
    arg: Bytes,
    opts: Vec<u8>,
}

impl<'a> Command<'a> {
    fn new(prev: &'a [u8], id: u16, arg: Bytes, opts: Vec<u8>) -> Self {
        Self {
            prev,
            id,
            arg,
            opts,
        }
    }
}

impl<'a> From<Command<'a>> for Bytes {
    fn from(this: Command<'a>) -> Bytes {
        let (header, footer) = (format!("[{},[", this.id), "]]");
        let len = header.len() + this.prev.len() + this.arg.len() + footer.len() + 2;
        let mut cmd = BytesMut::with_capacity(len);
        cmd.put(header);
        cmd.put(this.prev);
        if !this.prev.is_empty() {
            cmd.put(",");
        }
        cmd.put(&this.arg);
        if !this.opts.is_empty() && (!this.arg.is_empty() || !this.prev.is_empty()) {
            cmd.put(",");
        }
        cmd.put(footer);
        cmd.freeze()
    }
}
