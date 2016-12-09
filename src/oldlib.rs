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
extern crate scram;
extern crate parking_lot;
extern crate uuid;
extern crate serde;
extern crate serde_json;

pub mod errors;
pub mod commands;

use std::io::{Write, BufRead};
use std::io::Read;
use std::{str, result};
use std::fmt::Debug;
use std::thread;
use std::sync::mpsc::{self, SyncSender, Receiver};

use errors::*;
use ql2::types::Command;
use ql2::proto::{
    VersionDummy_Version as Version,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};

use serde::de::Deserialize;
pub use serde_json::{Value,
    from_iter, from_reader, from_slice, from_str, from_value, to_value,
    to_string, to_vec
};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use r2d2::{Pool, Config as PoolConfig, PooledConnection as PConn};
use parking_lot::RwLock;
use slog::{DrainExt, Logger, Record};
use slog_term::streamer;
use protobuf::ProtobufEnum;
use bufstream::BufStream;
use scram::{ClientFirst, ServerFirst, ServerFinal};

#[macro_export]
macro_rules! obj {
    ($( $key:ident: $val:expr ),* $(,)*) => {{
        use $crate::ToTerm;
        use ::std::collections::BTreeMap;

        let mut o = BTreeMap::new();
        $(
            let key = stringify!($key).to_string();
            let val = $val.to_term();
            o.insert(key, val);
        )*
        From::from(o)
    }}
}

#[macro_export]
macro_rules! arr {
    ($( $val:expr ),* $(,)*) => {{
        use $crate::ToTerm;

        let v = vec![$( $val.to_term(), )*];
        From::from(v)
    }}
}

// Macro to make it convenient to construct a `reql::error::Error`
// The macro also logs every error passed to it unless the error
// is marked as default.
macro_rules! error {
    ($e:expr) => {{
        let _logger = Client::logger().read();
        let error = Error::from($e);
        debug!(_logger, "Error({:?})", error);
        Err(error)
    }};
    // Do not log default errors otherwise our logs
    // will have misleading error messages.
    (default $e:expr) => {{
        let error = Error::from($e);
        Err(error)
    }}
}

// A custom `try` macro that makes use of our error macro to log errors and also any extra
// information sent to it. The idea is to log only when an error occurs and make sure we have all
// the infomation we need on our fingertips when that happens.
macro_rules! try {
    ($e:expr $(, $f:expr ),*) => {{
        match $e {
            Ok(res) => res,
            Err(err) => {
                let _logger = Client::logger().read();
                $(
                    info!(_logger, "{:?}", $f);
                )*
                return error!(err);
            },
        }
    }}
}

/// A ReQL Result
///
/// All public commands that can possibly return an error return this.
pub type Result<T> = result::Result<T, Error>;

/// ReQL Client
///
/// The entry point for all ReQL commands. All top level 
/// commands are implemented here.
pub struct Client;

/*
#[derive(Debug, Copy, Clone)]
enum CommandType {
    Read,
    Write,
    ChangeFeed,
}
*/

/// ReQL Response
///
/// Response returned by `run()`
pub type Response<T> = Result<ResponseValue<T>>;

/// Response value
#[derive(Debug, Clone)]
pub enum ResponseValue<T: Deserialize> {
    Write(WriteStatus),
    Read(T),
    Raw(Value),
}

const CHANNEL_SIZE: usize = 1024 * 1024;

impl ConnectionOpts {
    /// Sets servers
    pub fn set_servers(mut self, servers: Vec<&'static str>) -> ConnectionOpts {
        self.servers = servers;
        self
    }
    /// Sets database
    pub fn set_db(mut self, db: &'static str) -> ConnectionOpts {
        self.db = db;
        self
    }
    /// Sets username
    pub fn set_user(mut self, user: &'static str) -> ConnectionOpts {
        self.user = user;
        self
    }
    /// Sets password
    pub fn set_password(mut self, password: &'static str) -> ConnectionOpts {
        self.password = password;
        self
    }
    /// Sets retries
    pub fn set_retries(mut self, retries: u8) -> ConnectionOpts {
        self.retries = retries;
        self
    }

    /// Creates a connection pool
    pub fn connect(self) -> Result<()> {
        try!(Client::set_config(self.clone()));
        // If pool is already set we do nothing
        if Client::pool().read().is_some() {
            return Ok(());
        }
        // Otherwise we set it
        let mut pools: Vec<Pool<ConnectionManager>> = Vec::new();
        let mut opts = self;
        for s in &opts.servers[..] {
            opts.server = Some(s);
            let manager = ConnectionManager::new(opts.clone());
            let config = PoolConfig::default();
            let new_pool = try!(Pool::new(config, manager));
            pools.push(new_pool);
        }
        try!(Client::set_pool(pools));
        Ok(())
    }
}

impl Client {
    fn set_pool(p: Vec<Pool<ConnectionManager>>) -> Result<()> {
        let mut pool = POOL.write();
        *pool = Some(p);
        Ok(())
    }

    fn set_config(c: ConnectionOpts) -> Result<()> {
        let mut cfg = CONFIG.write();
        *cfg = c;
        Ok(())
    }

    pub fn connection(&self) -> ConnectionOpts {
        Self::config().read().clone()
    }

    pub fn run<T>(&self) -> Response<T>
        where T: 'static + Deserialize + Send + Debug
        {
            /*
            let mut query = self.to_term();
            if !query.info().db_set() {
                let config = Client::config().read();
                query = self.opt_arg("db", r.db(config.db)).to_term();
            }
            run::<T>(query)
                */
            unimplemented!();
        }

    pub fn run_with_opts<T, O>(&self, opts: O) -> Response<T>
        where T: 'static + Deserialize + Send + Debug, O: Into<ql2::types::Object>
        {
            let _opts = opts.into();
            //run::<T>(query.to_term())
            unimplemented!();
        }
}

fn run<T>(mut commands: String, opts: Option<String>) -> Response<T>
        where T: 'static + Deserialize + Send + Debug
{
    let (tx, rx) = mpsc::sync_channel(CHANNEL_SIZE);
    let sender = thread::Builder::new()
        .name("reql_command_run".to_string());
    if let Err(err) = sender.spawn(|| try!(send::<T>(commands, opts, tx))) {
        return error!(err);
    };
    Ok(rx)
}
