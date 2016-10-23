//! Primitives to Configure the Execution Environment

use parking_lot::RwLock;
use slog::{DrainExt, Logger};
use conn::{ConnectOpts, ConnectionManager};
use r2d2::{Pool, PooledConnection};
use errors::*;
use super::Result;
use slog_term;

lazy_static! {
    static ref POOL: RwLock<Option<Vec<Pool<ConnectionManager>>>> = RwLock::new(None);

    static ref LOGGER: RwLock<Logger> = RwLock::new(
                    Logger::root(
                        slog_term::streamer().full().build().fuse(),
                        o!("version" => env!("CARGO_PKG_VERSION"))
                        )
                    );

    static ref CONFIG: RwLock<ConnectOpts> = RwLock::new(ConnectOpts::default());
}

pub struct Client;

pub struct Config;

impl Client {
    pub fn logger() -> &'static RwLock<Logger> {
        &LOGGER
    }

    pub fn pool() -> &'static RwLock<Option<Vec<Pool<ConnectionManager>>>> {
        &POOL
    }

    pub fn config() -> &'static RwLock<ConnectOpts> {
        &CONFIG
    }

    pub fn conn() -> Result<PooledConnection<ConnectionManager>> {
        let logger = Self::logger().read();
        let cfg = Self::config().read();
        trace!(logger, "Calling Client::conn()");
        let pool = Self::pool().read();
        match *pool {
            Some(ref pool) => {
                let mut num_retries = cfg.retries;
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
        let mut pool = POOL.write();
        *pool = Some(p);
        Ok(())
    }

    pub fn set_config(c: ConnectOpts) -> Result<()> {
        let mut cfg = CONFIG.write();
        *cfg = c;
        Ok(())
    }
}
