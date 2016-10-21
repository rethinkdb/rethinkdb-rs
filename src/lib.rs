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
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate slog;
#[macro_use] extern crate quick_error;
extern crate slog_term;
extern crate protobuf;
extern crate scram;

pub mod conn;
pub mod types;
pub mod commands;
pub mod session;
pub mod errors;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[allow(non_upper_case_globals)]
pub const r: session::Client = session::Client;
