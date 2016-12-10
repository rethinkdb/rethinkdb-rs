//! Rust ReQL command reference
//!
//! Submit issues and pull requests to our [Github
//! repository](https://github.com/rust-rethinkdb/reql).

extern crate ql2;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate quick_error;
extern crate slog_term;
extern crate protobuf;
extern crate parking_lot;
extern crate uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod commands;

pub use ql2::{Result, types, conn, errors};

use parking_lot::RwLock;
use slog::{DrainExt, Logger, Record};
use slog_term::streamer;
use conn::ConnectionOpts;

#[derive(Debug, Clone, Copy)]
pub struct Pool;

lazy_static! {
    static ref LOGGER: RwLock<Logger> = RwLock::new({
        let drain = streamer().full().build();
        Logger::root(
            drain.fuse(),
            o!("source" =>
               move |info : &Record| {
                   format!("{}:{} {}", info.file(), info.line(), info.module())
               }))
    });

    static ref CONFIG: RwLock<ConnectionOpts> = RwLock::new(ConnectionOpts::default());
}

fn logger() -> &'static RwLock<Logger> {
    &LOGGER
}

fn config() -> &'static RwLock<ConnectionOpts> {
    &CONFIG
}

fn set_config(c: ConnectionOpts) {
    let mut cfg = CONFIG.write();
    *cfg = c;
}
