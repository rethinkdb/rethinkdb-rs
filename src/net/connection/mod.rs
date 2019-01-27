mod hand_shake;

use std::{
    str,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use hand_shake::HandShake;
use crate::{cmd::connect::Opts, Result};
use bytes::Bytes;
use futures::{channel::mpsc::UnboundedSender, lock::Mutex};
use romio::TcpStream;
use slab::Slab;

pub(crate) type RequestId = usize;
pub(crate) type Senders = Mutex<Slab<UnboundedSender<Bytes>>>;

/// The connection object returned by `r.connect()`
#[derive(Debug)]
pub struct Connection {
    db: String,
    stream: TcpStream,
    broken: AtomicBool,
    senders: Senders,
}

impl Connection {
    pub(crate) fn new(db: &str, stream: TcpStream) -> Self {
        Self {
            stream,
            db: db.to_owned(),
            broken: AtomicBool::new(false),
            senders: Senders::new(Slab::with_capacity(1024)),
        }
    }

    pub(crate) async fn hand_shake<'a>(self, opts: Opts<'a>) -> Result<Self> {
        await!(HandShake::new(self).greet(opts))
    }

    /// Change the default database on this connection
    ///
    /// **Example:** Change the default database so that we don’t need to
    /// specify the database when referencing a table.
    ///
    /// ```rust
    /// # use reql::r;
    /// # use futures::executor::block_on;
    /// # let mut conn = block_on(r.connect(())).unwrap();
    /// conn.use_db("marvel");
    /// r.table("heroes") // refers to r.db("marvel").table("heroes")
    /// # ;
    /// ```
    pub fn use_db(&mut self, name: &str) {
        self.db = name.to_owned();
    }

    pub fn broken(&self) -> bool {
        self.broken.load(SeqCst)
    }

    pub(crate) fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub(crate) fn senders(&self) -> &Senders {
        &self.senders
    }

    /*
    pub(crate) fn mark_broken(&self) {
        self.broken.store(true, SeqCst);
    }
    */

    pub(crate) fn db(&self) -> &str {
        &self.db
    }
}
