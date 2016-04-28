//! RethinkDB Driver

mod pool;

/// A connection to a RethinkDB database.
pub struct Connection;

pub struct Reql;

#[allow(non_upper_case_globals)]
pub const r: Reql = Reql{};
