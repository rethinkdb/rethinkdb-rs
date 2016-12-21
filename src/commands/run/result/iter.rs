use std::iter::{IntoIterator, Iterator};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use ql2::Encode;
use ::{Result, Client, Command};
use errors::*;
use conn::{
    ResponseValue,
    Request,
    Session,
};
use commands::RunOpts;
use commands::run::{
    Query, CHANNEL_SIZE,
};
use serde::Deserialize;

/// ReQL Response
///
/// Response returned by `run()`
pub struct Response<T: Deserialize>(Receiver<Result<ResponseValue<T>>>);

impl<T> Iterator for Response<T>
    where T: Deserialize + Send
{
    type Item = Result<ResponseValue<T>>;

    fn next(&mut self) -> Option<Result<ResponseValue<T>>> {
        match self.0.recv() {
            Ok(resp) => Some(resp),
            Err(_) => None,
        }
    }
}

impl<S, T> IntoIterator for Client<Query<S, T>, RunOpts>
    where S: 'static + Session + Send,
          T: 'static + Deserialize + Send,
{
    type Item = Result<ResponseValue<T>>;
    type IntoIter = Response<T>;

    fn into_iter(self) -> Response<T> {
        let (tx, rx) = mpsc::sync_channel::<Result<ResponseValue<T>>>(CHANNEL_SIZE);
        match self.errors {
            Some(errors) => {
                let errors = match Arc::try_unwrap(errors) {
                    Ok(errors) => errors,
                    Err(_) => {
                        let tx = tx.clone();
                        let msg = String::from("Failed to unwrap Arc");
                        let err = DriverError::Other(msg);
                        if let Err(err) = tx.send(err!(err)) {
                            error!("Failed to send message: {:?}", err);
                        }
                        return Response(rx);
                    },
                };
                if !errors.is_empty() {
                    for e in errors {
                        let tx = tx.clone();
                        if let Err(err) = tx.send(err!(e)) {
                            error!("Failed to send message: {:?}", err);
                        }
                    }
                } else {
                    // This is a bug so we want to know when it happens.
                    panic!("Expected at least one error but found 0. This is a bug.");
                }
            },
            None => {
                let sender = thread::Builder::new().name("reql_command_run".to_string());
                let res = sender.spawn(move || {
                    if let Err(err) = request(self.cmd, tx.clone()) {
                        if let Err(err) = tx.send(err!(err)) {
                            error!("Failed to send message: {:?}", err);
                        }
                    }
                });
                if let Err(err) = res {
                    error!("Failed to spawn a thread: {:?}", err);
                };
            },
        }
        Response(rx)
    }
}

fn request<S, T>(cmd: Command<Query<S, T>, RunOpts>, tx: mpsc::SyncSender<Result<ResponseValue<T>>>) -> Result<()>
    where S: Session + Send,
          T: Deserialize + Send
{
    let conn = cmd.0.sess;
    let mut req = Request::new(conn, tx)?;
    let ref cfg = ::config().read();
    let commands = cmd.0.term.encode();
    let opts = match cmd.1 {
        Some(ref opts) => Some(opts.encode()),
        None => None,
    };
    req.submit(cfg, commands, opts)
}
