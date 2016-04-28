//! RethinkDB connection pool

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate r2d2;

use std::io;

struct ConnectionManager;

#[allow(non_upper_case_globals)]
const r: ConnectionManager = ConnectionManager{};

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = ::Connection;
    type Error = io::Error;

    fn connect(&self) -> Result<::Connection, io::Error> {
        unimplemented!();
    }

    fn is_valid(&self, conn: &mut ::Connection) -> Result<(), io::Error> {
        unimplemented!();
    }

    fn has_broken(&self, conn: &mut ::Connection) -> bool {
        unimplemented!();
    }
}
