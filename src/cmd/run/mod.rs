mod arg;
mod opt;

use std::marker::PhantomData;

use crate::{
    cmd::*,
    net::{connection::Connection, response::session::Session},
};
use arg::Arg;
use bytes::Bytes;
use futures::channel::mpsc::UnboundedReceiver;
use serde::de::DeserializeOwned;

pub use opt::*;

macro_rules! runnable {
    ( $($cmd:ty,)* ) => {
        $(
            impl $cmd {
                pub fn run<'a, A, T>(self, arg: A) -> Run<'a, T>
                    where
                    A: Into<Arg<'a>>,
                    T: DeserializeOwned + 'static,
                    {
                        let Arg { conn, opts } = arg.into();
                        Run::new(self.bytes, conn, opts)
                    }
            }
        )*
    }
}

runnable! {
    expr::Expr,
    merge::Merge,
    table::Table,
}

#[derive(Debug)]
pub struct Run<'a, T> {
    pub(crate) conn: &'a Connection,
    pub(crate) query: Bytes,
    pub(crate) opts: Opts<'a>,
    pub(crate) session: Option<Session<'a>>,
    pub(crate) receiver: Option<UnboundedReceiver<Bytes>>,
    pub(crate) state: State,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub(crate) enum State {
    New,
    Initialised,
    SessionCreated,
    SessionWritten,
    SessionRead,
    Done,
}

impl<'a, T> Run<'a, T> {
    fn new(query: Bytes, conn: &'a Connection, mut opts: Opts<'a>) -> Self {
        if opts.db.is_none() {
            let db = conn.db();
            if !db.is_empty() {
                opts.db(db);
            }
        }
        Run {
            conn,
            query,
            opts,
            session: None,
            receiver: None,
            state: State::New,
            phantom: PhantomData,
        }
    }
}
