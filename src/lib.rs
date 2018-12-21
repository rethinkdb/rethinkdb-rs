#![feature(async_await, await_macro, futures_api)]

mod client;
mod error;

pub use crate::client::{Client, Config};

pub type Result<T> = std::result::Result<T, crate::error::Error>;
