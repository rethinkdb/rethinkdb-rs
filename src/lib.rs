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
//#[macro_use(o, slog_info, slog_log)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[macro_use]
extern crate quick_error;
extern crate protobuf;
extern crate parking_lot;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate futures;

#[macro_use]
mod macros;

pub mod commands;
pub mod types;

pub use commands::r;
pub use commands::run::{Run, RunWithConn};
pub use ql2::{Result, conn, errors};
pub use ql2::proto::Term;

use std::sync::Arc;

use conn::ConnectionOpts;
use parking_lot::RwLock;
use slog::Logger;
use slog_scope::set_global_logger;
use errors::Error;

#[derive(Debug, Clone)]
pub struct Command<T, O>(T, Option<O>);

#[derive(Debug, Clone)]
pub struct Client<T, O> {
    cmd: Command<T, O>,
    idx: u32,
    errors: Option<Arc<Vec<Error>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Pool;

lazy_static! {
    static ref CONFIG: RwLock<ConnectionOpts> = RwLock::new(ConnectionOpts::default());
}

fn config() -> &'static RwLock<ConnectionOpts> {
    &CONFIG
}

fn set_config(c: ConnectionOpts) {
    let mut cfg = CONFIG.write();
    *cfg = c;
}

pub fn set_logger(l: &Logger)
{
    set_global_logger(l.clone());
}
