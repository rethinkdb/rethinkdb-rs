//! RethinkDB connection pool

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate r2d2;

use std::io;
use conn;

struct ConnectionManager;

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = conn::Connection;
    type Error = io::Error;

    fn connect(&self) -> Result<conn::Connection, io::Error> {
        unimplemented!();
    }

    fn is_valid(&self, conn: &mut conn::Connection) -> Result<(), io::Error> {
        unimplemented!();
    }

    fn has_broken(&self, conn: &mut conn::Connection) -> bool {
        unimplemented!();
    }
}
