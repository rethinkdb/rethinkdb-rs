//! The Actual RethinkDB Commands

use errors::*;
use conn::{Connection, ConnectOpts, ConnectionManager};
use r2d2::{Pool, Config as PoolConfig};
use serde_json;
use ql2::proto;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use std::io::Read;
use std::str;
use protobuf::ProtobufEnum;
use byteorder::ReadBytesExt;
use super::session::Client;
use super::{Result, r};

pub struct RootCommand(Result<String>);
pub struct Command;
pub struct Query;

impl ConnectOpts {
    /// Sets servers
    pub fn set_servers(mut self, s: Vec<&'static str>) -> Self {
        self.servers = s;
        self
    }
    /// Sets database
    pub fn set_db(mut self, d: &'static str) -> Self {
        self.db = d;
        self
    }
    /// Sets username
    pub fn set_user(mut self, u: &'static str) -> Self {
        self.user = u;
        self
    }
    /// Sets password
    pub fn set_password(mut self, p: &'static str) -> Self {
        self.password = p;
        self
    }

    /// Creates a connection pool
    pub fn connect(self) -> Result<()> {
        let logger = try!(Client::logger().read());
        trace!(logger, "Calling r.connect()");
        // If pool is already set we do nothing
        if try!(Client::pool().read()).is_some() {
            info!(logger, "A connection pool is already initialised. We will use that one instead...");
            return Ok(());
        }
        // Otherwise we set it
        let manager = ConnectionManager::new(self);
        let config = PoolConfig::builder()
            // If we are under load and our pool runs out of connections
            // we are doomed so we set a very high number of maximum
            // connections that can be opened
            .pool_size(1000000)
            // To counter the high number of open connections we set
            // a reasonable number of minimum connections we want to
            // keep when we are idle.
            .min_idle(Some(10))
            .build();
        let new_pool = try!(Pool::new(config, manager));
        try!(Client::set_pool(new_pool));
        info!(logger, "A connection pool has been initialised...");
        Ok(())
    }
}

impl Client {
    pub fn connection(&self) -> ConnectOpts {
        ConnectOpts::default()
    }

    pub fn db(&self, name: &str) -> RootCommand {
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::DB,
                    format!("{:?}", name),
                    None,
                    None
                    )))
    }

    pub fn table(&self, name: &str) -> RootCommand {
        r.db("test").table(name)
    }

    pub fn object(&self) -> serde_json::builder::ObjectBuilder {
        serde_json::builder::ObjectBuilder::new()
    }

    pub fn array(&self) -> serde_json::builder::ArrayBuilder {
        serde_json::builder::ArrayBuilder::new()
    }
}

impl RootCommand {
    pub fn table(self, name: &str) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::TABLE,
                    format!("{:?}", name),
                    None,
                    Some(commands)
                    )))
    }

    pub fn insert(self, expr: serde_json::Value) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        let data = match serde_json::to_string(&expr) {
            Ok(f) => f,
            Err(e) => return RootCommand(Err(From::from(DriverError::Json(e)))),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::INSERT,
                    data,
                    None,
                    Some(commands),
                    )))
    }

    pub fn filter(self, expr: serde_json::Value) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        let filter = match serde_json::to_string(&expr) {
            Ok(f) => f,
            Err(e) => return RootCommand(Err(From::from(DriverError::Json(e)))),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::FILTER,
                    filter,
                    None,
                    Some(commands),
                    )))
    }

    pub fn run(self) -> Result<String> {
        let logger = try!(Client::logger().read());
        trace!(logger, "Calling r.run()");
        let commands = try!(self.0);
        let pool = try!(Client::pool().read());
        if let Some(ref pool) = *pool {
            let mut conn = try!(pool.get());
            conn.token += 1;
            let query = Query::wrap(
                proto::Query_QueryType::START,
                Some(commands),
                None);
            debug!(logger, "{}", query);
            try!(Query::write(&query, &mut conn));
            let resp = try!(Query::read(&mut conn));
            let resp = try!(str::from_utf8(&resp));
            debug!(logger, "{}", resp);
        } else {
            let msg = String::from("Your connection pool is not initialised. \
                                   Use `r.connection().connect()` to initialise the pool \
                                   before trying to send any connections to the database. \
                                   This is typically done in the `main` function.");
            return Err(From::from(ConnectionError::Other(msg)));
        }
        Ok(String::new())
    }
}

impl Command {
    pub fn wrap(command: proto::Term_TermType, arguments: String, options: Option<String>, commands: Option<String>) -> String {
        let mut cmds = format!("[{},", command.value());
        let args: String;
        if let Some(commands) = commands {
            args = format!("{},{}", commands, arguments);
        } else {
            args = arguments;
        }
        cmds.push_str(format!("[{}]", args).as_str());
        if let Some(options) = options {
            cmds.push_str(format!(",{{{}}}", options).as_str());
        }
        cmds.push(']');
        cmds
    }
}

impl Query {
    pub fn wrap(query_type: proto::Query_QueryType, query: Option<String>, options: Option<String>) -> String {
        let mut qry = format!("[{}", query_type.value());
        if let Some(query) = query {
            qry.push_str(format!(",{}", query).as_str());
        }
        if let Some(options) = options {
            qry.push_str(format!(",{}", options).as_str());
        }
        qry.push_str("]");
        qry
    }

    pub fn write(query: &str, conn: &mut Connection) -> Result<()> {
        let query = query.as_bytes();
        let token = conn.token;
        if let Err(error) = conn.stream.write_u64::<LittleEndian>(token) {
            conn.broken = true;
            return Err(From::from(error));
        }
        if let Err(error) = conn.stream.write_u32::<LittleEndian>(query.len() as u32) {
            conn.broken = true;
            return Err(From::from(error));
        }
        if let Err(error) = conn.stream.write_all(query) {
            conn.broken = true;
            return Err(From::from(error));
        }
        if let Err(error) = conn.stream.flush() {
            conn.broken = true;
            return Err(From::from(error));
        }
        Ok(())
    }

    pub fn read(conn: &mut Connection) -> Result<Vec<u8>> {
            // @TODO use response_token to implement parallel reads and writes?
            // let response_token = try!(conn.stream.read_u64::<LittleEndian>());
            let _ = match conn.stream.read_u64::<LittleEndian>() {
                Ok(token) => token,
                Err(error) => {
                    conn.broken = true;
                    return Err(From::from(error));
                },
            };
            let len = match conn.stream.read_u32::<LittleEndian>() {
                Ok(len) => len,
                Err(error) => {
                    conn.broken = true;
                    return Err(From::from(error));
                },
            };
            let mut resp = vec![0u8; len as usize];
            if let Err(error) = conn.stream.read_exact(&mut resp) {
                    conn.broken = true;
                    return Err(From::from(error));
            }
            Ok(resp)
    }
}
