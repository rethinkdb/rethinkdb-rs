//! Primitives to Configure the Execution Environment

use std::sync::RwLock;
use slog::{DrainExt, Logger};
use conn::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use errors::*;
use super::Result;
use slog_term;
use std::error::Error as StdError;

lazy_static! {
    static ref POOL: RwLock<Option<Vec<Pool<ConnectionManager>>>> = RwLock::new(None);

    static ref LOGGER: RwLock<Logger> = RwLock::new(
                    Logger::root(
                        slog_term::streamer().full().build().fuse(),
                        o!("version" => env!("CARGO_PKG_VERSION"))
                        )
                    );
}

pub struct Client;

pub struct Config;

impl Client {
    //pub fn logger() -> RwLock<Logger> {
    pub fn logger() -> &'static RwLock<Logger> {
        &LOGGER
    }

    pub fn pool() -> &'static RwLock<Option<Vec<Pool<ConnectionManager>>>> {
        &POOL
    }

    pub fn conn() -> Result<PooledConnection<ConnectionManager>> {
        let pool = try!(Self::pool().read());
        match *pool {
            Some(ref pool) => {
                let conn = try!(pool[0].get());
                return Ok(conn);
            },
            None => {
                let msg = String::from("Your connection pool is not initialised. \
                                   Use `r.connection().connect()` to initialise the pool \
                                   before trying to send any connections to the database. \
                                   This is typically done in the `main` function.");
                return Err(From::from(ConnectionError::Other(msg)));
            },
        }
    }

    pub fn set_pool(p: Vec<Pool<ConnectionManager>>) -> Result<()> {
        match POOL.write() {
            Ok(mut pool) => {
                *pool = Some(p);
                Ok(())
            },
            Err(err) => return Err(From::from(DriverError::Lock(err.description().to_string()))),
        }
    }
}
