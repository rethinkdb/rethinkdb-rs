//! RethinkDB Traits and Errors

pub mod error;
pub mod conn;

pub trait Reql {
    fn connect<T: conn::Connection>(&self, opts: conn::Opts) -> T;
}
