mod arg;
mod opt;

use self::arg::Arg;
use crate::{
    cmd::*,
    err,
    net::{
        connection::Connection,
        response::{message::Message, session::Session, Response},
    },
    Result,
};
use bytes::{BufMut, Bytes, BytesMut};
use futures::{channel::oneshot, prelude::*};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub use self::opt::*;

macro_rules! runnable {
    ( $($cmd:ty,)* ) => {
        $(
            impl $cmd {
                pub fn run<'a, A, T>(self, arg: A) -> impl Future<Output=Result<Response<T>>> + 'a
                    where
                    A: Into<Arg<'a>>,
                    T: DeserializeOwned + 'static,
                    {
                        let Arg { conn, opts } = arg.into();
                        run(self.bytes, conn, opts)
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

fn run<'a, T>(
    query: Bytes,
    conn: &'a Connection,
    mut opts: Opts<'a>,
) -> impl Future<Output = Result<Response<T>>> + 'a
where
    T: DeserializeOwned,
{
    async move {
        if opts.db.is_none() {
            let db = conn.db();
            if !db.is_empty() {
                opts.db(db);
            }
        }
        // We can't use `ser::to_vec` here because it will wrap the DB term in
        // an array. Luckily, the options to `run` do not contain arrays so we
        // can safely use the upstream `to_vec` here.
        let opts = serde_json::to_vec(&opts)?;
        let opts_len = opts.len();
        let (header, sep, footer) = ("[1,", ",", "]");
        let len = header.len() + query.len() + sep.len() + opts_len + footer.len();
        let mut msg = BytesMut::with_capacity(len);
        msg.put(header);
        msg.put(query);
        // don't include an empty object
        if opts_len > 2 {
            msg.put(sep);
            msg.put(opts);
        }
        msg.put(footer);
        let (sender, reciever) = oneshot::channel();
        let id = {
            let mut senders = await!(conn.senders().lock());
            senders.insert(sender)
        };
        let session = Session::new(id, conn);
        await!(session.write(&msg))?;
        // wait for data
        await!(session.read())?;
        let resp = await!(reciever)?;
        let msg: Message<_> = match serde_json::from_slice(&resp) {
            Ok(msg) => msg,
            Err(_) => {
                let msg: Message<Value> = serde_json::from_slice(&resp)?;
                return Err(err::Driver::UnexpectedResponse(msg.r))?;
            }
        };
        msg.is_valid()?;
        Ok(Response::new(msg.r, msg.p.unwrap_or_default()))
    }
}
