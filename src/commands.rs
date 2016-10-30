//! The Actual RethinkDB Commands

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use std::io::{Write, BufRead};
use std::io::Read;
use std::{str, result};
use std::error::Error as StdError;
use std::net::TcpStream;

use super::r;
use super::errors::*;
use ql2::proto::{self, Term_TermType as tt, Query_QueryType as qt};
use serde_json::{self, Value};
use serde_json::builder::{ObjectBuilder, ArrayBuilder};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use r2d2::{self, Pool, Config as PoolConfig, PooledConnection};
use parking_lot::RwLock;
use slog::{DrainExt, Logger};
use slog_term;
use protobuf::ProtobufEnum;
use bufstream::BufStream;
use scram::{ClientFirst, ServerFirst, ServerFinal};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct RootCommand(Result<String>);
pub struct Command;
pub struct Query;

lazy_static! {
    static ref POOL: RwLock<Option<Vec<Pool<ConnectionManager>>>> = RwLock::new(None);

    static ref LOGGER: RwLock<Logger> = RwLock::new({
        let drain = slog_term::streamer().compact().build().fuse();
        Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")))
    });

    static ref CONFIG: RwLock<ConnectOpts> = RwLock::new(ConnectOpts::default());
}

/// Options
#[derive(Debug, Clone)]
pub struct ConnectOpts {
    pub servers: Vec<&'static str>,
    pub db: &'static str,
    pub user: &'static str,
    pub password: &'static str,
    pub retries: u8,
    pub ssl: Option<SslCfg>,
    server: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct SslCfg {
    pub ca_certs: &'static str,
}

impl Default for ConnectOpts {
    fn default() -> ConnectOpts {
        ConnectOpts {
            servers: vec!["localhost:28015"],
            db: "test",
            user: "admin",
            password: "",
            retries: 5,
            ssl: None,
            server: None,
        }
    }
}

/// A connection to a RethinkDB database.
#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub token: u64,
    pub broken: bool,
}

impl ConnectOpts {
    /// Sets servers
    pub fn set_servers(mut self, servers: Vec<&'static str>) -> Self {
        self.servers = servers;
        self
    }
    /// Sets database
    pub fn set_db(mut self, db: &'static str) -> Self {
        self.db = db;
        self
    }
    /// Sets username
    pub fn set_user(mut self, user: &'static str) -> Self {
        self.user = user;
        self
    }
    /// Sets password
    pub fn set_password(mut self, password: &'static str) -> Self {
        self.password = password;
        self
    }
    /// Sets retries
    pub fn set_retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    /// Creates a connection pool
    pub fn connect(self) -> Result<()> {
        let logger = Client::logger().read();
        trace!(logger, "Calling r.connect()");
        try!(Client::set_config(self.clone()));
        // If pool is already set we do nothing
        if Client::pool().read().is_some() {
            info!(logger,
                  "A connection pool is already initialised. We will use that one instead...");
            return Ok(());
        }
        // Otherwise we set it
        let mut pools: Vec<Pool<ConnectionManager>> = Vec::new();
        let mut opts = self;
        for s in &opts.servers[..] {
            opts.server = Some(s);
            let manager = ConnectionManager::new(opts.clone());
            let config = PoolConfig::builder()
                // If we are under load and our pool runs out of connections
                // we are doomed so we set a very high number of maximum
                // connections that can be opened
                .pool_size(100)
                // To counter the high number of open connections we set
                // a reasonable number of minimum connections we want to
                // keep when we are idle.
                .min_idle(Some(10))
                .build();
            let new_pool = try!(Pool::new(config, manager));
            pools.push(new_pool);
        }
        try!(Client::set_pool(pools));
        info!(logger, "A connection pool has been initialised...");
        Ok(())
    }
}

impl Connection {
    pub fn new(opts: &ConnectOpts) -> Result<Connection> {
        let server = match opts.server {
            Some(server) => server,
            None => {
                return Err(From::from(ConnectionError::Other(String::from("No server selected."))))
            }
        };
        let mut conn = Connection {
            stream: try!(TcpStream::connect(server)),
            token: 0,
            broken: false,
        };
        let _ = try!(conn.handshake(opts));
        Ok(conn)
    }

    fn handshake(&mut self, opts: &ConnectOpts) -> Result<()> {
        // Send desired version to the server
        let _ = try!(self.stream
            .write_u32::<LittleEndian>(proto::VersionDummy_Version::V1_0 as u32));
        try!(parse_server_version(&self.stream));

        // Send client first message
        let (scram, msg) = try!(client_first(opts));
        let _ = try!(self.stream.write_all(&msg[..]));

        // Send client final message
        let (scram, msg) = try!(client_final(scram, &self.stream));
        let _ = try!(self.stream.write_all(&msg[..]));

        // Validate final server response and flush the buffer
        try!(parse_server_final(scram, &self.stream));
        let _ = try!(self.stream.flush());

        Ok(())
    }
}

fn parse_server_version(stream: &TcpStream) -> Result<()> {
    let logger = Client::logger().read();
    let resp = try!(parse_server_response(stream));
    let info: ServerInfo = match serde_json::from_str(&resp) {
        Ok(res) => res,
        Err(err) => {
            crit!(logger, "{}", err);
            return Err(From::from(err));
        }
    };

    if !info.success {
        return Err(From::from(ConnectionError::Other(resp.to_string())));
    };
    Ok(())
}

fn parse_server_response(stream: &TcpStream) -> Result<String> {
    let logger = Client::logger().read();
    // The server will then respond with a NULL-terminated string response.
    // "SUCCESS" indicates that the connection has been accepted. Any other
    // response indicates an error, and the response string should describe
    // the error.
    let mut resp = Vec::new();
    let mut buf = BufStream::new(stream);
    let _ = try!(buf.read_until(b'\0', &mut resp));

    let _ = resp.pop();

    if resp.is_empty() {
        let msg = String::from("unable to connect for an unknown reason");
        crit!(logger, "{}", msg);
        return Err(From::from(ConnectionError::Other(msg)));
    };

    let resp = try!(str::from_utf8(&resp)).to_string();
    // If it's not a JSON object it's an error
    if !resp.starts_with("{") {
        crit!(logger, "{}", resp);
        return Err(From::from(ConnectionError::Other(resp)));
    };
    Ok(resp)
}

fn client_first(opts: &ConnectOpts) -> Result<(ServerFirst, Vec<u8>)> {
    let logger = Client::logger().read();
    let scram = try!(ClientFirst::new(opts.user, opts.password, None));
    let (scram, client_first) = scram.client_first();

    let ar = AuthRequest {
        protocol_version: 0,
        authentication_method: String::from("SCRAM-SHA-256"),
        authentication: client_first,
    };
    let mut msg = match serde_json::to_vec(&ar) {
        Ok(res) => res,
        Err(err) => {
            crit!(logger, "{}", err);
            return Err(From::from(err));
        }
    };
    msg.push(b'\0');
    Ok((scram, msg))
}

fn client_final(scram: ServerFirst, stream: &TcpStream) -> Result<(ServerFinal, Vec<u8>)> {
    let logger = Client::logger().read();
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = match serde_json::from_str(&resp) {
        Ok(res) => res,
        Err(err) => {
            crit!(logger, "{}", err);
            return Err(From::from(err));
        }
    };

    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return Err(From::from(DriverError::Auth(err)));
        } else {
            return Err(From::from(ConnectionError::Other(err)));
        }
    };
    if let Some(auth) = info.authentication {
        let scram = scram.handle_server_first(&auth).unwrap();
        let (scram, client_final) = scram.client_final();
        let auth = AuthConfirmation { authentication: client_final };
        let mut msg = match serde_json::to_vec(&auth) {
            Ok(res) => res,
            Err(err) => {
                crit!(logger, "{}", err);
                return Err(From::from(err));
            }
        };
        msg.push(b'\0');
        Ok((scram, msg))
    } else {
        Err(From::from(ConnectionError::Other(String::from("Server did not send authentication \
                                                            info."))))
    }
}

fn parse_server_final(scram: ServerFinal, stream: &TcpStream) -> Result<()> {
    let logger = Client::logger().read();
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = match serde_json::from_str(&resp) {
        Ok(res) => res,
        Err(err) => {
            crit!(logger, "{}", err);
            return Err(From::from(err));
        }
    };
    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return Err(From::from(DriverError::Auth(err)));
        } else {
            return Err(From::from(ConnectionError::Other(err)));
        }
    };
    if let Some(auth) = info.authentication {
        let _ = scram.handle_server_final(&auth).unwrap();
    }
    Ok(())
}

pub struct ConnectionManager(ConnectOpts);

impl ConnectionManager {
    pub fn new(opts: ConnectOpts) -> Self {
        ConnectionManager(opts)
    }
}

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        Connection::new(&self.0)
    }

    fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
        let logger = Client::logger().read();
        conn.token += 1;
        let query = Query::wrap(proto::Query_QueryType::START, Some("1"), None);
        try!(Query::write(&query, &mut conn));
        let resp = try!(Query::read(&mut conn));
        let resp = try!(str::from_utf8(&resp));
        if resp != r#"{"t":1,"r":[1]}"# {
            warn!(logger,
                  "Got {} from server instead of the expected `is_valid()` response.",
                  resp);
            return Err(From::from(ConnectionError::Other(String::from("Unexpected response \
                                                                       from server."))));
        }
        Ok(())
    }

    fn has_broken(&self, conn: &mut Connection) -> bool {
        if conn.broken {
            return true;
        }
        match conn.stream.take_error() {
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
                let msg = String::from("Failed to get a connection.");
                let mut last_error = Err(From::from(ConnectionError::Other(msg)));
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
                        let msg = String::from("All servers are currently down.");
                        last_error = Err(From::from(ConnectionError::Other(msg)));
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
                return Err(From::from(ConnectionError::Other(msg)));
            }
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
        // Arrays are a special case: since ReQL commands are sent as arrays
        // We have to wrap them using MAKE_ARRAY
        let val = wrap_arrays(self.clone());
        match serde_json::to_string(&val) {
            Ok(cmd) => Ok((Some(cmd), None)),
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

macro_rules! define {
    (impl IntoCommandArg for $T:ty) => {
        impl IntoCommandArg for $T {
            fn to_arg(&self) -> Result<(Option<String>, Option<String>)> {
                Ok((Some(self.to_string()), None))
            }
        }
    }
}

define!{ impl IntoCommandArg for bool }
define!{ impl IntoCommandArg for char }
define!{ impl IntoCommandArg for u8 }
define!{ impl IntoCommandArg for u16 }
define!{ impl IntoCommandArg for u32 }
define!{ impl IntoCommandArg for u64 }
define!{ impl IntoCommandArg for usize }
define!{ impl IntoCommandArg for i8 }
define!{ impl IntoCommandArg for i16 }
define!{ impl IntoCommandArg for i32 }
define!{ impl IntoCommandArg for i64 }
define!{ impl IntoCommandArg for isize }
define!{ impl IntoCommandArg for f32 }
define!{ impl IntoCommandArg for f64 }

macro_rules! command {
    ($name:ident, $cmd:ident) => {
        pub fn $name<T>(self, arg: T) -> RootCommand
            where T: IntoCommandArg
            {
                let commands = match self.0 {
                    Ok(t) => t,
                    Err(e) => return RootCommand(Err(e)),
                };
                RootCommand(Command::wrap(tt::$cmd,
                                          Some(arg),
                                          Some(&commands)))
            }
    };
    ($name:ident, $cmd:ident, root_cmd) => {
        pub fn $name<T>(self, arg: T) -> RootCommand
            where T: IntoCommandArg
            {
                RootCommand(Command::wrap(tt::$cmd,
                                          Some(arg),
                                          None))
            }
    };
    ($name:ident, $cmd:ident, no_args) => {
        pub fn $name(self) -> RootCommand {
                let commands = match self.0 {
                    Ok(t) => t,
                    Err(e) => return RootCommand(Err(e)),
                };
                RootCommand(Command::wrap(tt::$cmd,
                                          None as Option<&str>,
                                          Some(&commands)))
            }
    };
}

impl Client {
    command!(db, DB, root_cmd);
    command!(db_create, DB_CREATE, root_cmd);
    command!(db_drop, DB_DROP, root_cmd);

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

    pub fn object(&self) -> ObjectBuilder {
        ObjectBuilder::new()
    }

    pub fn array(&self) -> ArrayBuilder {
        ArrayBuilder::new()
    }
}

impl RootCommand {
    command!(table_create, TABLE_CREATE);
    command!(table_drop, TABLE_DROP);
    command!(table, TABLE);
    command!(index_drop, INDEX_DROP);
    command!(index_create, INDEX_CREATE);
    command!(replace, REPLACE);
    command!(update, UPDATE);
    command!(order_by, ORDER_BY);
    command!(without, WITHOUT);
    command!(contains, CONTAINS);
    command!(limit, LIMIT);
    command!(get, GET);
    command!(get_all, GET_ALL);
    command!(filter, FILTER);
    command!(insert, INSERT);
    command!(delete, DELETE, no_args);

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

fn wrap_arrays(mut val: Value) -> Value {
    if val.is_array() {
        let mut array = Vec::with_capacity(2);
        array.push(Value::I64(tt::MAKE_ARRAY.value() as i64));
        if let Value::Array(vec) = val {
            let mut new_val = Vec::with_capacity(vec.len());
            for v in vec.into_iter() {
                if v.is_array() || v.is_object() {
                    new_val.push(wrap_arrays(v));
                } else {
                    new_val.push(v)
                }
            }
            val = Value::Array(new_val);
        }
        array.push(val);
        val = Value::Array(array);
    } else if val.is_object() {
        if let Value::Object(mut obj) = val {
            for (k, v) in obj.clone() {
                obj.insert(k, wrap_arrays(v));
            }
            val = Value::Object(obj);
        }
    }
    val
}
