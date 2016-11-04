//! Prelude

pub use command::Response;
pub use futures::Future;
pub use futures::stream::Stream;
pub use serde_json::{Value,
    from_iter, from_reader, from_slice, from_str, from_value, to_value,
    to_string, to_vec
};

// Consumers to make blocking more convenient
//
// Ideally we would want to use just one trait `Consumer`.
// However, the implementations for `T: Stream` and `T: Future`
// would clash.

/// Block the current thread until we have received all the results from
/// the stream.
pub trait StreamConsumer {
    fn consume(self);
}

/// Block the current thread until we have received all the results from
/// the future.
pub trait FutureConsumer {
    fn consume(self);
}

macro_rules! consume {
    () => {
        fn consume(self) {
            for _ in self.wait() { }
        }
    }
}

impl<T: Stream> StreamConsumer for T { consume!{} }
impl<T: Future> FutureConsumer for T { consume!{} }
