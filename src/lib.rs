//! RethinkDB Driver
//!
//! ```rust
//! use reql::r;
//! use reql::conn::Opts;
//!
//! let mut conn = r.connect(Default::default());
//! ```

extern crate ql2;
extern crate byteorder;
extern crate bufstream;

pub mod conn;
pub mod error;
pub mod pool;

use conn::{Connection, Opts};

pub struct Reql;

#[allow(non_upper_case_globals)]
pub const r: Reql = Reql;

impl Reql {
    pub fn connect(&self, opts: Opts) -> Connection {
        Connection::new(opts)
    }
}
