//! Rust RethinkDB Driver
//!
//! ```rust
//! extern crate reql;
//!
//! use reql::r;
//!
//! # fn main() {
//! r.connection().connect().unwrap();
//! # }
//! ```

extern crate ql2;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
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
extern crate scram;
extern crate parking_lot;
extern crate uuid;
extern crate futures;

pub mod prelude;
pub mod command;
pub mod error;

/// The top-level ReQL namespace
#[allow(non_upper_case_globals)]
pub const r: command::Client = command::Client;
