mod arg;
mod opt;

use {
    crate::{r, Client},
    bytes::Bytes,
};

pub use arg::Arg;

fn table(prev: &Bytes, arg: Arg) -> Client {
    Client::new(prev, 15, arg)
}

impl r {
    pub fn table<A>(&self, arg: A) -> Client
    where
        A: Into<Arg>,
    {
        table(&Bytes::new(), arg.into())
    }
}

impl Client {
    pub fn table<A>(&self, arg: A) -> Client
    where
        A: Into<Arg>,
    {
        table(&self.0, arg.into())
    }
}
