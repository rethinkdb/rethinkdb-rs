//! RethinkDB Driver

mod pool;

/// A connection to a RethinkDB database.
pub struct Connection;

struct Reql;

#[allow(non_upper_case_globals)]
const r: Reql = Reql{};
