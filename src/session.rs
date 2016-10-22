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
        let logger = try!(Self::logger().read());
        trace!(logger, "Calling Client::conn()");
        let pool = try!(Self::pool().read());
        match *pool {
            Some(ref pool) => {
                let mut num_retries = 10;
                let mut last_error = Err(
                    From::from(
                        ConnectionError::Other(
                            String::from("Failed to get a connection")
                            )));
                while num_retries > 0 {
                    let mut least_connections = 0;
                    let mut least_connected_server = 0;
                    let mut most_idle = 0;
                    let mut most_idle_server = 0;
                    for (i, p) in pool.iter().enumerate() {
                        let state = p.state();
                        if least_connections == 0 || least_connections > state.connections {
                            least_connections = state.connections;
                            least_connected_server = i
                        }
                        if most_idle == 0 || most_idle < state.idle_connections {
                            most_idle = state.idle_connections;
                            most_idle_server = i
                        }
                    }
                    if most_idle > 0 {
                        match pool[most_idle_server].get() {
                            Ok(conn) => return Ok(conn),
                            Err(error) => last_error = Err(From::from(error)),
                        }
                    } else if least_connections > 0 {
                        match pool[least_connected_server].get() {
                            Ok(conn) => return Ok(conn),
                            Err(error) => last_error = Err(From::from(error)),
                        }
                    } else {
                        last_error = Err(
                            From::from(
                                ConnectionError::Other(
                                    String::from("All servers are currently down...")
                                    )));
                    }
                    num_retries -= 1;
                }
                return last_error;
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
