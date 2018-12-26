#![feature(async_await, await_macro, futures_api, integer_atomics)]

pub mod cmd;
pub mod error;
pub mod opt;
pub(crate) mod qry;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[allow(non_camel_case_types)]
pub struct r;

pub use crate::cmd::Connection;
