//! The Actual RethinkDB Commands

use errors::*;
use conn::{Connection, ConnectOpts};
use serde_json::{self, Value};
use ql2::proto::{Term_TermType as tt, Query_QueryType as qt};
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};
use std::io::Read;
use std::str;
use protobuf::ProtobufEnum;
use byteorder::ReadBytesExt;
use super::session::Client;
use super::{Result, r};
use std::error::Error as StdError;

#[derive(Debug)]
pub struct RootCommand(Result<String>);
pub struct Command;
pub struct Query;

pub trait IntoCommandArg {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)>;
}

impl<'a> IntoCommandArg for &'a str {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(format!("{:?}", self)), None))
    }
}

impl IntoCommandArg for String {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(format!("{:?}", self)), None))
    }
}

impl IntoCommandArg for Value {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        match serde_json::to_string(self) {
            Ok(mut cmd) => {
                // Arrays are a special case: since ReQL commands are sent as arrays
                if self.is_array() {
                    // We have to wrap them using MAKE_ARRAY
                    cmd = format!("[{},{}]", tt::MAKE_ARRAY.value(), cmd);
                }
                Ok((Some(cmd), None))
            },
            Err(e) => Err(From::from(DriverError::Json(e))),
        }
    }
}

impl<T> IntoCommandArg for (T, Value)
        where T: IntoCommandArg
{
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        if !self.1.is_object() {
            let msg = String::from("Only objects are allowed as function options. You should use `r.object()` to pass optional arguments in your functions.");
            return Err(From::from(DriverError::Other(msg)));
        }
        let arg = try!(self.0.to_arg());
        let opt = try!(self.1.to_arg());
        Ok((arg.0, opt.0))
    }
}

impl IntoCommandArg for RootCommand {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        match self.0 {
            Ok(ref cmd) => Ok((Some(cmd.to_string()), None)),
            Err(ref e) => Err(From::from(DriverError::Other(e.description().to_string()))),
        }
    }
}

impl IntoCommandArg for bool {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for char {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for u8 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for u16 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for u32 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for u64 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for usize {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for i8 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for i16 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for i32 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for i64 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for isize {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for f32 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}
impl IntoCommandArg for f64 {
    fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
        Ok((Some(self.to_string()), None))
    }
}

impl Client {
    pub fn connection(&self) -> ConnectOpts {
        Self::config().read().clone()
    }

    pub fn expr<T>(&self, e: T) -> RootCommand
        where T: IntoCommandArg
    {
        match e.to_arg() {
            Ok(arg) => if let Some(arg) = arg.0 {
                RootCommand(Ok(arg))
            } else {
                RootCommand(Ok(String::new()))
            },
            Err(e) => RootCommand(Err(e)),
        }
    }

    pub fn db_create<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        RootCommand(Command::wrap(tt::DB_CREATE,
                                     Some(name),
                                     None))
    }

    pub fn db_drop<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        RootCommand(Command::wrap(tt::DB_DROP,
                                     Some(name),
                                     None))
    }

    pub fn db<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        RootCommand(Command::wrap(tt::DB,
                                     Some(name),
                                     None))
    }

    pub fn table_create<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let config = Client::config().read();
        r.db(config.db).table_create(name)
    }

    pub fn table<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let config = Client::config().read();
        r.db(config.db).table(name)
    }

    pub fn table_drop<T>(&self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let config = Client::config().read();
        r.db(config.db).table_drop(name)
    }

    pub fn object(&self) -> serde_json::builder::ObjectBuilder {
        serde_json::builder::ObjectBuilder::new()
    }

    pub fn array(&self) -> serde_json::builder::ArrayBuilder {
        serde_json::builder::ArrayBuilder::new()
    }
}

impl RootCommand {
    pub fn table_create<T>(self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::TABLE_CREATE,
                                     Some(name),
                                     Some(&commands)))
    }

    pub fn table<T>(self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::TABLE,
                                     Some(name),
                                     Some(&commands)))
    }

    pub fn table_drop<T>(self, name: T) -> RootCommand
        where T: IntoCommandArg
    {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::TABLE_DROP,
                                     Some(name),
                                     Some(&commands)))
    }

    pub fn insert<T>(self, data: T) -> RootCommand
        where T: IntoCommandArg
    {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::INSERT,
                                     Some(data),
                                     Some(&commands)))
    }

    pub fn delete(self) -> RootCommand {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::DELETE,
                                  None as Option<&str>,
                                  Some(&commands)))
    }

    pub fn filter<T>(self, filter: T) -> RootCommand
        where T: IntoCommandArg
    {
        let commands = match self.0 {
            Ok(t) => t,
            Err(e) => return RootCommand(Err(e)),
        };
        RootCommand(Command::wrap(tt::FILTER,
                                     Some(filter),
                                     Some(&commands)))
    }

    pub fn run(self) -> Result<String> {
        let logger = Client::logger().read();
        trace!(logger, "Calling r.run()");
        let commands = try!(self.0);
        let cfg = Client::config().read();
        let query = Query::wrap(qt::START, Some(&commands), None);
        debug!(logger, "{}", query);
        let mut conn = try!(Client::conn());
        // Try sending the query
        {
            let mut i = 0;
            let mut write = true;
            let mut connect = false;
            while i < cfg.retries {
                debug!(logger, "Getting connection...");
                if connect {
                    drop(&mut conn);
                    conn = match Client::conn() {
                        Ok(c) => c,
                        Err(error) => {
                            if i == cfg.retries - 1 {
                                // The last error
                                return Err(From::from(error));
                            } else {
                                debug!(logger, "Failed getting a connection. Retrying...");
                                i += 1;
                                continue;
                            }
                        }
                    };
                }
                debug!(logger, "Connection aquired.");
                conn.token += 1;
                if write {
                    debug!(logger, "Writing query...");
                    if let Err(error) = Query::write(&query, &mut conn) {
                        connect = true;
                        if i == cfg.retries - 1 {
                            // The last error
                            return Err(From::from(error));
                        } else {
                            debug!(logger, "Failed to write query. Retrying...");
                            i += 1;
                            continue;
                        }
                    }
                    debug!(logger, "Query written...");
                    connect = false;
                }
                debug!(logger, "Reading query...");
                match Query::read(&mut conn) {
                    Ok(resp) => {
                        let result = try!(str::from_utf8(&resp));
                        debug!(logger, "{}", result);
                        // If the write operation failed, retry it
                        // {"t":18,"e":4100000,"r":["Cannot perform write: primary replica for
                        // shard [\"\", +inf) not available"],"b":[]}
                        let msg = r#"{"t":18,"e":4100000,"r":["Cannot perform write: primary replica for shard"#;
                        if result.starts_with(msg) {
                            write = true;
                            if i == cfg.retries - 1 {
                                // The last error
                                return Err(
                                From::from(
                                    AvailabilityError::OpFailed(
                                        String::from("Not available")
                                        )));
                            } else {
                                debug!(logger, "Write operation failed. Retrying...");
                                i += 1;
                                continue;
                            }
                        } else {
                            debug!(logger, "Query successfully read.");
                            // This is a successful operation
                            break;
                        }
                    }
                    Err(error) => {
                        write = false;
                        if i == cfg.retries - 1 {
                            // The last error
                            return Err(From::from(error));
                        } else {
                            debug!(logger, "Failed to read query. Retrying...");
                            i += 1;
                            continue;
                        }
                    }
                }
            }
        }
        Ok(String::new())
    }
}

impl Command {
    pub fn wrap<T>(command: tt,
                input: Option<T>,
                commands: Option<&str>)
                -> Result<String>
        where T: IntoCommandArg
        {
            let mut cmds = format!("[{},", command.value());
            let mut args = String::new();
            if let Some(commands) = commands {
                args.push_str(commands);
                if input.is_some() {
                    args.push_str(",");
                }
            }
            let mut arguments: Option<String> = None;
            let mut options: Option<String> = None;
            if let Some(input) = input {
                let (args, opts) = try!(input.to_arg());
                arguments = args;
                options = opts;
            }
            if let Some(arguments) = arguments {
                args.push_str(&arguments);
            }
            cmds.push_str(&format!("[{}]", args));
            if let Some(options) = options {
                cmds.push_str(&format!(",{}", options));
            }
            cmds.push(']');
            Ok(cmds)
        }
}

impl Query {
    pub fn wrap(query_type: qt,
                query: Option<&str>,
                options: Option<&str>)
        -> String {
            let mut qry = format!("[{}", query_type.value());
            if let Some(query) = query {
                qry.push_str(&format!(",{}", query));
            }
            if let Some(options) = options {
                qry.push_str(&format!(",{}", options));
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
            }
        };
        let len = match conn.stream.read_u32::<LittleEndian>() {
            Ok(len) => len,
            Err(error) => {
                conn.broken = true;
                return Err(From::from(error));
            }
        };
        let mut resp = vec![0u8; len as usize];
        if let Err(error) = conn.stream.read_exact(&mut resp) {
            conn.broken = true;
            return Err(From::from(error));
        }
        Ok(resp)
    }
}
