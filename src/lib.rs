#![feature(async_await, await_macro, futures_api, integer_atomics)]

pub mod cmd;
pub mod err;

/// The top-level ReQL namespace
#[allow(non_camel_case_types)]
pub struct r;

pub use crate::cmd::connect::Connection;

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, self::err::Error>;
