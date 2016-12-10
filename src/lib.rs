//! Rust ReQL command reference
//!
//! Submit issues and pull requests to our [Github
//! repository](https://github.com/rust-rethinkdb/reql).

extern crate ql2;
extern crate r2d2;
extern crate byteorder;
extern crate bufstream;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate quick_error;
extern crate slog_term;
extern crate protobuf;
extern crate parking_lot;
extern crate uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod commands;

pub use ql2::{Result, types, conn, errors};

use errors::*;
use conn::{ConnectionOpts, Connection, Request};

use parking_lot::RwLock;
use r2d2::{Pool, Config, PooledConnection as PConn};
use slog::{DrainExt, Logger, Record};
use slog_term::streamer;

lazy_static! {
    static ref POOL: RwLock<Option<Vec<Pool<ConnectionManager>>>> = RwLock::new(None);

    static ref LOGGER: RwLock<Logger> = RwLock::new({
        let drain = streamer().full().build();
        Logger::root(
            drain.fuse(),
            o!("source" =>
               move |info : &Record| {
                   format!("{}:{} {}", info.file(), info.line(), info.module())
               }))
    });

    static ref CONFIG: RwLock<ConnectionOpts> = RwLock::new(ConnectionOpts::default());
}

pub struct ConnectionManager;

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        let opts = config().read();
        Connection::new(&opts)
    }

    fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
        conn.incr_token();
        unimplemented!();
        /*
        let query = wrap_query(QueryType::START, Some(String::from("1")), None);
        try!(write_query(&query, &mut conn));
        let resp = try!(read_query(&mut conn));
        let resp: ReqlResponse = try!(from_slice(&resp[..]));
        if let Some(respt) = ResponseType::from_i32(resp.t) {
            if let ResponseType::SUCCESS_ATOM = respt {
                let val: Vec<i32> = try!(from_value(resp.r.clone()));
                if val == [1] {
                    return Ok(());
                }
            }
        }
        let msg = format!("Unexpected response from server: {:?}", resp);
        error!(ConnectionError::Other(msg))
        */
    }

    fn has_broken(&self, conn: &mut Connection) -> bool {
        if conn.broken() {
            return true;
        }
        match conn.stream().take_error() {
            Ok(error) => {
                if error.is_some() {
                    return true;
                }
            }
            Err(_) => {
                return true;
            }
        }
        false
    }
}

pub type PooledConnection = PConn<ConnectionManager>;

fn logger() -> &'static RwLock<Logger> {
    &LOGGER
}

fn pool() -> &'static RwLock<Option<Vec<Pool<ConnectionManager>>>> {
    &POOL
}

fn config() -> &'static RwLock<ConnectionOpts> {
    &CONFIG
}

fn connection() -> Result<PooledConnection> {
    let cfg = config().read();
    let pool = pool().read();
    match *pool {
        Some(ref pool) => {
            let msg = String::from("Failed to get a connection.");
            let mut last_error = error!(ConnectionError::Other(msg));
            macro_rules! return_conn {
                ($e:expr) => {{
                    match $e {
                        Ok(mut conn) => {
                            conn.incr_token();
                            return Ok(conn);
                        },
                        Err(error) => last_error = error!(error),
                    }
                }}
            }
            let mut num_retries = cfg.retries();
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
                    return_conn!(pool[most_idle_server].get());
                } else if least_connections > 0 {
                    return_conn!(pool[least_connected_server].get());
                } else {
                    let msg = String::from("All servers are currently down.");
                    last_error = error!(ConnectionError::Other(msg));
                }
                num_retries -= 1;
            }
            return last_error;
        }
        None => {
            let msg = String::from("Your connection pool is not initialised. \
                                   Use `r.connection().connect()` to initialise the pool \
                                   before trying to send any connections to the database. \
                                   This is typically done in the `main` function.");
            return error!(ConnectionError::Other(msg));
        }
    }
}
