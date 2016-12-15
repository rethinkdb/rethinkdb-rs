use std::thread;

use futures::sync::mpsc::{self, Receiver};
use futures::{Future, Sink};
use ::Result;
use conn::{
    ResponseValue,
    Session,
};
use commands::{
    Client, RunOpts,
};
use commands::run::{
    Query, CHANNEL_SIZE,
};
use serde::Deserialize;

/// ReQL Response
///
/// Response returned by `run()`
pub type Response<T> = Receiver<Result<ResponseValue<T>>>;

impl<S, T> Client<Query<S, T>, RunOpts>
    where S: 'static + Session + Send,
          T: 'static + Deserialize + Send,
{
    pub fn into_stream(self) -> Response<T> {
        let (tx, rx) = mpsc::channel::<Result<ResponseValue<T>>>(CHANNEL_SIZE);
        match self.errors {
            Some(errors) => {
                if !errors.is_empty() {
                    for e in errors {
                        let tx = tx.clone();
                        if let Err(err) = tx.send(err!(e)).wait() {
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
                    for res in self {
                        let tx = tx.clone();
                        if let Err(err) = tx.send(res).wait() {
                            error!("Failed to send message: {:?}", err);
                        }
                    }
                });
                if let Err(err) = res {
                    error!("Failed to spawn a thread: {:?}", err);
                };
            }
        }
        rx
    }
}
