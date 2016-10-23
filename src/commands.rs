//! The Actual RethinkDB Commands

use errors::*;
use conn::{Connection, ConnectionManager, ConnectOpts};
use serde_json;
use ql2::proto;
use r2d2::PooledConnection;
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

impl Client {
    pub fn connection(&self) -> ConnectOpts {
        Self::config().read().clone()
    }

    pub fn db_create(&self, name: &str) -> RootCommand {
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::DB_CREATE,
                    format!("{:?}", name),
                    None,
                    None
                    )))
    }

    pub fn db_drop(&self, name: &str) -> RootCommand {
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::DB_DROP,
                    format!("{:?}", name),
                    None,
                    None
                    )))
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

    pub fn table_create(&self, name: &str) -> RootCommand {
        let config = Client::config().read();
        r.db(config.db).table_create(name)
    }

    pub fn table(&self, name: &str) -> RootCommand {
        let config = Client::config().read();
        r.db(config.db).table(name)
    }

    pub fn object(&self) -> serde_json::builder::ObjectBuilder {
        serde_json::builder::ObjectBuilder::new()
    }

    pub fn array(&self) -> serde_json::builder::ArrayBuilder {
        serde_json::builder::ArrayBuilder::new()
    }
}

impl RootCommand {
    pub fn table_create(self, name: &str) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Ok(
                Command::wrap(
                    proto::Term_TermType::TABLE_CREATE,
                    format!("{:?}", name),
                    None,
                    Some(commands)
                    )))
    }

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
        let logger = Client::logger().read();
        trace!(logger, "Calling r.run()");
        let commands = try!(self.0);
        let cfg = Client::config().read();
        let mut conn: PooledConnection<ConnectionManager> = {
            let res = || -> Result<PooledConnection<ConnectionManager>> {
                let mut i = 0;
                while i < cfg.retries {
                    match Client::conn() {
                        Ok(c) => {
                            return Ok(c);
                        },
                        Err(error) => {
                            if i == cfg.retries-1 { // The last error
                                return Err(From::from(error));
                            }
                        },
                    }
                    i += 1;
                };
                // This line shouldn't be reachable.
                // Without it the compiler thinks `not all control paths return a value`
                Err(From::from(DriverError::Other(String::from("Unreachable!"))))
            }();
            try!(res)
        };
        conn.token += 1;
        let query = Query::wrap(
            proto::Query_QueryType::START,
            Some(commands),
            None);
        debug!(logger, "{}", query);
        // Try sending the query
        {
            let mut i = 0;
            let mut write = true;
            while i < cfg.retries {
                if write {
                    if let Err(error) = Query::write(&query, &mut conn) {
                            if i == cfg.retries-1 { // The last error
                                return Err(From::from(error));
                            } else {
                                continue;
                            }
                    }
                }
                match Query::read(&mut conn) {
                    Ok(resp) => {
                        let result = try!(str::from_utf8(&resp));
                        debug!(logger, "{}", result);
                        // If the write operation failed, retry it
                        // {"t":18,"e":4100000,"r":["Cannot perform write: primary replica for
                        // shard [\"\", +inf) not available"],"b":[]}
                        if result.starts_with(r#"{"t":18,"e":4100000,"r":["Cannot perform write: primary replica for shard"#) {
                            write = true;
                            if i == cfg.retries-1 { // The last error
                                return Err(
                                    From::from(
                                        AvailabilityError::OpFailed(
                                            String::from("Not available")
                                            )));
                            }
                        } else {
                            // This is a successful operation
                            break;
                        }
                    },
                    Err(error) => {
                        write = false;
                        if i == cfg.retries-1 { // The last error
                            return Err(From::from(error));
                        }
                    },
                }
                i += 1;
            }
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
