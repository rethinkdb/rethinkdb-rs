//! Prelude

use std::ops::FnMut;

use futures::stream;

pub use command::Response;
pub use futures::Future;
pub use futures::stream::Stream;
pub use serde_json::{Value, from_str, to_string};

pub trait Consumer {
    fn consume(self);
}

macro_rules! consume {
    () => {
        fn consume(self) {
            for _ in self.wait() { }
        }
    }
}

impl<T> Consumer for Response<T> { consume!{} }
impl<S, F> Consumer for stream::ForEach<S, F>
    where S: Stream, F: FnMut(S::Item) -> Result<(), S::Error>
{ consume!{} }
