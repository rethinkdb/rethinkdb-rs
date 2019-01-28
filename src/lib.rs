#![feature(
    async_await,
    await_macro,
    futures_api,
    integer_atomics,
    decl_macro,
    try_trait
)]

pub mod cmd;
pub mod err;
pub(crate) mod net;
pub(crate) mod ser;

/// The top-level ReQL namespace
#[allow(non_camel_case_types)]
pub struct r;

/// The database cluster client
#[derive(Debug, Clone)]
pub struct Client(bytes::Bytes);

pub use crate::net::{
    connection::Connection,
    response::{profile::Profile, Response},
};

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, err::Error>;
