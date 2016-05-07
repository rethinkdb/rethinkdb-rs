//! RethinkDB Traits and Errors

pub mod error;
pub mod conn;

pub trait R {
    type Connection;
    fn connect(&self, opts: conn::Opts) -> Self::Connection;
}
