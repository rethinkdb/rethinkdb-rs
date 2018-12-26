#![feature(async_await, await_macro, futures_api, integer_atomics)]

mod client;
mod conn;
pub mod error;
pub(crate) mod proto;

pub use crate::{
    client::{Client, Config},
    conn::Connection,
};

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[cfg(test)]
mod tests {
    use crate::Client;
    use futures::executor::block_on;

    #[test]
    fn driver_can_connect() {
        let mut r = Client::new();
        block_on(r.connect()).unwrap();
    }
}
