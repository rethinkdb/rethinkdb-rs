mod arg;
mod opt;

use {
    crate::{
        net::{connection::Connection, response::session::Session},
        Client,
    },
    arg::Arg,
    bytes::Bytes,
    futures::channel::mpsc::UnboundedReceiver,
    serde::de::DeserializeOwned,
    std::marker::PhantomData,
};

pub use opt::*;

impl Client {
    pub fn run<'a, A, T>(self, arg: A) -> Run<'a, T>
    where
        A: Into<Arg<'a>>,
        T: DeserializeOwned + 'static,
    {
        let Arg { conn, opts } = arg.into();
        Run::new(self.0, conn, opts)
    }
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
