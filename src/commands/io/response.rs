use std::sync::mpsc::TryRecvError;

use {Response, ResponseValue};
use futures::stream::Stream;
use futures::{Poll, Async};
use serde::Deserialize;
use errors::Error;

impl<T> Stream for Response<T>
where T: Deserialize + Send + 'static
{
    type Item = ResponseValue<T>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.0.try_recv() {
            Ok(msg) => {
                msg.and_then(|val| Ok(Async::Ready(Some(val))))
            }
            Err(error) => {
                match error {
                    TryRecvError::Empty => Ok(Async::NotReady),
                    TryRecvError::Disconnected => Ok(Async::Ready(None)),
                }
            }
        }
    }
}
