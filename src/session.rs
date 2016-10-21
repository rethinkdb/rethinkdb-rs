//! Primitives to Configure the Execution Environment

use std::sync::RwLock;
use slog::{DrainExt, Logger};
use conn::ConnectionManager;
use r2d2::Pool;
use errors::*;
use super::Result;
use slog_term;
use std::error::Error as StdError;

lazy_static! {
    static ref POOL: RwLock<Option<Pool<ConnectionManager>>> = RwLock::new(None);

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

    //pub fn pool() -> RwLock<Option<Pool<ConnectionManager>>> {
    pub fn pool() -> &'static RwLock<Option<Pool<ConnectionManager>>> {
        &POOL
    }

    pub fn set_pool(p: Pool<ConnectionManager>) -> Result<()> {
        match POOL.write() {
            Ok(mut pool) => {
                *pool = Some(p);
                Ok(())
            },
            Err(err) => return Err(From::from(DriverError::Lock(err.description().to_string()))),
        }
    }
}
