//! Command Reference

use std::io::{Write, BufRead};
use std::io::Read;
use std::{str, result};
use std::error::Error as StdError;
use std::net::TcpStream;
use std::fmt::Debug;
use std::thread;
use std::collections::BTreeMap;

use super::r;
use error::*;
use prelude::*;
use ql2::proto::{self,
    Term_TermType as TT,
    Query_QueryType as QT,
    Response_ErrorType as ET,
    Response_ResponseType as RT
};

use serde::de::Deserialize;
use serde_json::builder::{ObjectBuilder, ArrayBuilder};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use r2d2::{self, Pool, Config as PoolConfig, PooledConnection as PConn};
use parking_lot::RwLock;
use slog::{DrainExt, Logger, Record};
use slog_term::streamer;
use protobuf::ProtobufEnum;
use bufstream::BufStream;
use scram::{ClientFirst, ServerFirst, ServerFinal};
use futures::{finished, BoxFuture};
use futures::stream::{self, Receiver, Sender as StreamSender};

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

macro_rules! error {
    ($e:expr) => {{
        let _logger = Client::logger().read();
        let error = Error::from($e);
        debug!(_logger, "Err({:?})", error);
        Err(error)
    }};
    // Do not log default errors otherwise our logs
    // will have misleading error messages.
    (default $e:expr) => {{
        let error = Error::from($e);
        Err(error)
    }}
}

macro_rules! try {
    ($e:expr $(, $f:expr ),*) => {{
        match $e {
            Ok(res) => res,
            Err(err) => {
                let _logger = Client::logger().read();
                $(
                    debug!(_logger, "{:?}", $f);
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

/// A JSON Object
pub type Object = BTreeMap<String, Value>;

/// ReQL Client
///
/// The entry point for all ReQL commands. All top level 
/// commands are implemented here.
pub struct Client;

/// ReQL Command
///
/// Implements all other ReQL commands that are not implemented on the
/// `Client`.
#[derive(Debug)]
pub struct Command(Result<String>);

/// ReQL Response
///
/// Response returned by `run()`
pub type Response<T> = Receiver<ResponseValue<T>, Error>;

/// Response value
#[derive(Debug, Clone)]
pub enum ResponseValue<T: Deserialize> {
    Write(WriteStatus),
    Read(T),
    Raw(Value),
}

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

    static ref CONFIG: RwLock<ConnectOpts> = RwLock::new(ConnectOpts::default());
}

/// Connection Options
///
/// Implements methods for configuring details to connect to database servers.
#[derive(Debug, Clone)]
pub struct ConnectOpts {
    servers: Vec<&'static str>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    ssl: Option<SslCfg>,
    server: Option<&'static str>,
}

#[derive(Debug, Clone)]
struct SslCfg {
    ca_certs: &'static str,
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
struct Connection {
    stream: TcpStream,
    token: u64,
    broken: bool,
}

impl ConnectOpts {
    /// Sets servers
    pub fn set_servers(mut self, servers: Vec<&'static str>) -> ConnectOpts {
        self.servers = servers;
        self
    }
    /// Sets database
    pub fn set_db(mut self, db: &'static str) -> ConnectOpts {
        self.db = db;
        self
    }
    /// Sets username
    pub fn set_user(mut self, user: &'static str) -> ConnectOpts {
        self.user = user;
        self
    }
    /// Sets password
    pub fn set_password(mut self, password: &'static str) -> ConnectOpts {
        self.password = password;
        self
    }
    /// Sets retries
    pub fn set_retries(mut self, retries: u8) -> ConnectOpts {
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

impl Connection {
    pub fn new(opts: &ConnectOpts) -> Result<Connection> {
        let server = match opts.server {
            Some(server) => server,
            None => {
                return error!(ConnectionError::Other(String::from("No server selected.")))
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
    let resp = try!(parse_server_response(stream));
    let info: ServerInfo = try!(from_str(&resp));
    if !info.success {
        return error!(ConnectionError::Other(resp.to_string()));
    };
    Ok(())
}

fn parse_server_response(stream: &TcpStream) -> Result<String> {
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
        return error!(ConnectionError::Other(msg));
    };

    let resp = try!(str::from_utf8(&resp)).to_string();
    // If it's not a JSON object it's an error
    if !resp.starts_with("{") {
        return error!(ConnectionError::Other(resp));
    };
    Ok(resp)
}

fn client_first(opts: &ConnectOpts) -> Result<(ServerFirst, Vec<u8>)> {
    let scram = try!(ClientFirst::new(opts.user, opts.password, None));
    let (scram, client_first) = scram.client_first();

    let ar = AuthRequest {
        protocol_version: 0,
        authentication_method: String::from("SCRAM-SHA-256"),
        authentication: client_first,
    };
    let mut msg = try!(to_vec(&ar));
    msg.push(b'\0');
    Ok((scram, msg))
}

fn client_final(scram: ServerFirst, stream: &TcpStream) -> Result<(ServerFinal, Vec<u8>)> {
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = try!(from_str(&resp));

    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return error!(DriverError::Auth(err));
        } else {
            return error!(ConnectionError::Other(err));
        }
    };

    if let Some(auth) = info.authentication {
        let scram = scram.handle_server_first(&auth).unwrap();
        let (scram, client_final) = scram.client_final();
        let auth = AuthConfirmation { authentication: client_final };
        let mut msg = try!(to_vec(&auth));
        msg.push(b'\0');
        Ok((scram, msg))
    } else {
        error!(ConnectionError::Other(String::from("Server did not send authentication \
                                                            info.")))
    }
}

fn parse_server_final(scram: ServerFinal, stream: &TcpStream) -> Result<()> {
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = try!(from_str(&resp));
    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return error!(DriverError::Auth(err));
        } else {
            return error!(ConnectionError::Other(err));
        }
    };
    if let Some(auth) = info.authentication {
        let _ = try!(scram.handle_server_final(&auth));
    }
    Ok(())
}

struct ConnectionManager(ConnectOpts);

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
        conn.token += 1;
        let query = wrap_query(QT::START, Some("1"), None);
        try!(write_query(&query, &mut conn));
        let resp = try!(read_query(&mut conn));
        let resp: ReqlResponse = try!(from_slice(&resp[..]));
        if let Some(respt) = RT::from_i32(resp.t) {
            if let RT::SUCCESS_ATOM = respt {
                let val: Vec<i32> = try!(from_value(resp.r.clone()));
                if val == [1] {
                    return Ok(());
                }
            }
        }
        let msg = format!("Unexpected response from server: {:?}", resp);
        error!(ConnectionError::Other(msg))
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

/// A serialised ReQL command argument
pub type Argument = Option<String>;

/// A serialised ReQL options map
pub type Options = Option<String>;

/// A type that can be passed into a ReQL command
pub trait IntoCommandArg {
    /// Defines how a type can be safely passed into a command.
    ///
    /// A successful result returns a tuple as (argument, options).
    /// An argument is the main argument that is mandatory for 
    /// commands that do accept at least one argument. Options
    /// is a map of optional options that a command can accept.
    ///
    /// Both arguments and options are returned as [serialised] 
    /// strings.
    ///
    /// [serialised]: https://rethinkdb.com/docs/writing-drivers/#serializing-queries
    fn to_arg(&self) -> Result<(Argument, Options)>;
}

impl<'a> IntoCommandArg for &'a str {
    fn to_arg(&self) -> Result<(Argument, Options)> {
        Ok((Some(format!("{:?}", self)), None))
    }
}

impl IntoCommandArg for String {
    fn to_arg(&self) -> Result<(Argument, Options)> {
        Ok((Some(format!("{:?}", self)), None))
    }
}

impl IntoCommandArg for Value {
    fn to_arg(&self) -> Result<(Argument, Options)> {
        // Arrays are a special case: since ReQL commands are sent as arrays
        // We have to wrap them using MAKE_ARRAY
        let val = wrap_arrays(self.clone());
        let cmd = try!(to_string(&val));
        Ok((Some(cmd), None))
    }
}

impl<T> IntoCommandArg for (T, Value)
where T: IntoCommandArg
{
    fn to_arg(&self) -> Result<(Argument, Options)> {
        if !self.1.is_object() {
            let msg = String::from("Only objects are allowed as function options. You should use `r.object()` to pass optional arguments in your functions.");
            return Err(From::from(DriverError::Other(msg)));
        }
        let arg = try!(self.0.to_arg());
        let opt = try!(self.1.to_arg());
        Ok((arg.0, opt.0))
    }
}

impl IntoCommandArg for Command {
    fn to_arg(&self) -> Result<(Argument, Options)> {
        match self.0 {
            Ok(ref cmd) => Ok((Some(cmd.to_string()), None)),
            Err(ref e) => error!(DriverError::Other(e.description().to_string())),
        }
    }
}

impl IntoCommandArg for () {
    fn to_arg(&self) -> Result<(Argument, Options)> {
        Ok((None, None))
    }
}

macro_rules! define {
    (impl IntoCommandArg for $T:ty) => {
        impl IntoCommandArg for $T {
            fn to_arg(&self) -> Result<(Argument, Options)> {
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
        pub fn $name<T>(self, arg: T) -> Command
            where T: IntoCommandArg
            {
                let commands = match self.0 {
                    Ok(t) => t,
                    Err(e) => return Command(Err(e)),
                };
                Command(wrap_command(TT::$cmd,
                                     Some(arg),
                                     Some(&commands)))
            }
    };
    ($name:ident, $cmd:ident, root_cmd) => {
        pub fn $name<T>(self, arg: T) -> Command
            where T: IntoCommandArg
            {
                Command(wrap_command(TT::$cmd,
                                     Some(arg),
                                     None))
            }
    };
    ($name:ident, $cmd:ident, no_args) => {
        pub fn $name(self) -> Command {
            let commands = match self.0 {
                Ok(t) => t,
                Err(e) => return Command(Err(e)),
            };
            Command(wrap_command(TT::$cmd,
                                 None as Option<&str>,
                                 Some(&commands)))
        }
    };
}

type PooledConnection = PConn<ConnectionManager>;

impl Client {
    command!(db, DB, root_cmd);
    command!(db_create, DB_CREATE, root_cmd);
    command!(db_drop, DB_DROP, root_cmd);

    fn logger() -> &'static RwLock<Logger> {
        &LOGGER
    }

    fn pool() -> &'static RwLock<Option<Vec<Pool<ConnectionManager>>>> {
        &POOL
    }

    fn config() -> &'static RwLock<ConnectOpts> {
        &CONFIG
    }

    fn conn() -> Result<PooledConnection> {
        let cfg = Self::config().read();
        let pool = Self::pool().read();
        match *pool {
            Some(ref pool) => {
                let msg = String::from("Failed to get a connection.");
                let mut last_error = error!(default ConnectionError::Other(msg));
                macro_rules! return_conn {
                    ($e:expr) => {{
                        match $e {
                            Ok(mut conn) => {
                                conn.token += 1;
                                return Ok(conn);
                            },
                            Err(error) => last_error = error!(error),
                        }
                    }}
                }
                let mut num_retries = cfg.retries;
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

    fn set_pool(p: Vec<Pool<ConnectionManager>>) -> Result<()> {
        let mut pool = POOL.write();
        *pool = Some(p);
        Ok(())
    }

    fn set_config(c: ConnectOpts) -> Result<()> {
        let mut cfg = CONFIG.write();
        *cfg = c;
        Ok(())
    }

    pub fn connection(&self) -> ConnectOpts {
        Self::config().read().clone()
    }

    pub fn expr<T>(&self, e: T) -> Command
        where T: IntoCommandArg
        {
            match e.to_arg() {
                Ok(arg) => if let Some(arg) = arg.0 {
                    Command(Ok(arg))
                } else {
                    Command(Ok(String::new()))
                },
                Err(e) => Command(error!(e)),
            }
        }

    pub fn table_create<T>(&self, name: T) -> Command
        where T: IntoCommandArg
        {
            let config = Client::config().read();
            r.db(config.db).table_create(name)
        }

    pub fn table<T>(&self, name: T) -> Command
        where T: IntoCommandArg
        {
            let config = Client::config().read();
            r.db(config.db).table(name)
        }

    pub fn table_drop<T>(&self, name: T) -> Command
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

impl Command {
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
    command!(changes, CHANGES, no_args);

    pub fn run<T>(self) -> Result<Response<T>>
        where T: 'static + Deserialize + Send + Debug
        {
            let (tx, rx) = stream::channel();
            let commands = try!(self.0);
            let sender = thread::Builder::new()
                .name("reql_command_run".to_string());
            if let Err(err) = sender.spawn(|| send::<T>(commands, tx).wait()) {
                return error!(err);
            };
            Ok(rx)
        }

    pub fn run_with_opts<T>(self, _opts: Object) -> Result<Response<T>>
        where T: 'static + Deserialize + Send + Debug
        {
            let (tx, rx) = stream::channel();
            let commands = try!(self.0);
            let sender = thread::Builder::new()
                .name("reql_command_run".to_string());
            if let Err(err) = sender.spawn(|| send::<T>(commands, tx).wait()) {
                return error!(err);
            };
            Ok(rx)
        }
}

type Sender<T> = StreamSender<ResponseValue<T>, Error>;

fn send<T>(commands: String, mut tx: Sender<T>) -> BoxFuture<(), ()>
where T: 'static + Deserialize + Send + Debug
{
    macro_rules! return_error {
        ($e:expr) => {{
            let error = error!($e);
            let _ = tx.send(error).wait();
            return finished(()).boxed();
        }}
    }
    macro_rules! try {
        ($e:expr) => {{
            match $e {
                Ok(value) => value,
                Err(err) => return_error!(err),
            }
        }}
    }
    let cfg = Client::config().read();
    let mut query = wrap_query(QT::START, Some(&commands), None);
    let mut conn = try!(Client::conn());
    // Try sending the query
    {
        let mut i = 0;
        let mut write = true;
        let mut connect = false;
        while i < cfg.retries {
            // Open a new connection if necessary
            if connect {
                drop(&mut conn);
                conn = match Client::conn() {
                    Ok(c) => c,
                    Err(error) => {
                        if i == cfg.retries - 1 {
                            return_error!(error);
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                };
            }
            // Submit the query if necessary
            if write {
                if let Err(error) = write_query(&query, &mut conn) {
                    connect = true;
                    if i == cfg.retries - 1 {
                        return_error!(error);
                    } else {
                        i += 1;
                        continue;
                    }
                }
                connect = false;
            }
            // Handle the response
            let (new_tx, tx_returned, write_opt, retry, res) = process_response::<T>(&mut query, &mut conn, tx);
            tx = new_tx;
            if let Err(error) = res {
                    write = write_opt;
                    if i == cfg.retries - 1 || !retry {
                        return_error!(error);
                    }
                    if !tx_returned {
                        return_error!(error);
                    } else {
                        i += 1;
                        continue;
                    }
            }
            break;
        }
    }
    finished(()).boxed()
}

fn process_response<T>(query: &mut String, conn: &mut PooledConnection, mut tx: Sender<T>) -> (Sender<T>, bool, bool, bool, Result<()>)
    where T: 'static + Deserialize + Send + Debug
{
    let mut write = false;
    let mut retry = false;
    let (new_tx, tx_returned, new_retry, res) = handle_response::<T>(conn, tx);
    tx = new_tx;
    macro_rules! return_error {
        ($e:expr) => {{
            let error = $e;
            return (tx, tx_returned, write, retry, error!(default error));
        }}
    }
    macro_rules! try {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return_error!(error),
            }
        }}
    }
    match res {
        Ok(t) => {
            match t {
                RT::SUCCESS_ATOM | RT::SUCCESS_SEQUENCE | RT::WAIT_COMPLETE | RT::SERVER_INFO | RT::CLIENT_ERROR | RT::COMPILE_ERROR | RT::RUNTIME_ERROR  => {/* we are done */},
                RT::SUCCESS_PARTIAL => {
                    *query = wrap_query(QT::CONTINUE, None, None);
                    if let Err(error) = write_query(query, conn) {
                        write = true;
                        retry = true;
                        return_error!(error);
                    }
                    let (new_tx, _, _, new_retry, res) = process_response::<T>(query, conn, tx);
                    tx = new_tx;
                    retry = new_retry;
                    if let Err(error) = res {
                        return_error!(error);
                    }
                },
            }
        }
        Err(error) => {
            retry = new_retry;
            match error {
                Error::Runtime(error) => {
                    match error {
                        RuntimeError::Availability(error) => {
                            match error {
                                AvailabilityError::OpFailed(msg) => {
                                    if msg.starts_with("Cannot perform write: primary replica for shard") {
                                        write = true;
                                        retry = true;
                                    }
                                    return_error!(AvailabilityError::OpFailed(msg));
                                }
                                error => return_error!(error),
                            }
                        }
                        error => return_error!(error),
                    }
                }
                error => return_error!(error),
            }
        }
    }
    (tx, tx_returned, write, retry, Ok(()))
}

fn handle_response<T>(conn: &mut PooledConnection, mut tx: Sender<T>) -> (Sender<T>, bool, bool, Result<RT>)
    where T: 'static + Deserialize + Send + Debug
{
    let (new_tx, _) = stream::channel();
    let mut retry = false;
    macro_rules! return_error {
        ($e:expr) => {{
            let error = $e;
            return (tx, true, retry, error!(default error));
        }}
    }
    macro_rules! try {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return_error!(error),
            }
        }}
    }
    macro_rules! try_tx {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return (new_tx, false, retry, error!(default error)),
            }
        }}
    }
    match read_query(conn) {
        Ok(resp) => {
            let result: ReqlResponse = try!(from_slice(&resp[..]));
            let respt: RT;
            if let Some(t) = RT::from_i32(result.t) {
                respt = t;
            } else {
                let msg = format!("Unsupported response type ({}), returned by the database.", result.t);
                return_error!(DriverError::Other(msg));
            }
            // If the database says this response is an error convert the error 
            // message to our native one.
            let has_generic_error = match respt {
                RT::CLIENT_ERROR | RT::COMPILE_ERROR | RT::RUNTIME_ERROR => true,
                _ => false,
            };
            let mut msg = String::new();
            if result.e.is_some() || has_generic_error {
                msg = if let Value::Array(error) = result.r.clone() {
                    if error.len() == 1 {
                        if let Some(Value::String(msg)) = error.into_iter().next() {
                            msg
                        } else {
                            return_error!(ResponseError::Db(result.r));
                        }
                    } else {
                        return_error!(ResponseError::Db(result.r));
                    }
                } else {
                    return_error!(ResponseError::Db(result.r));
                };
            }
            if let Some(e) = result.e {
                if let Some(error) = ET::from_i32(e) {
                    match error {
                        ET::INTERNAL => return_error!(RuntimeError::Internal(msg)),
                        ET::RESOURCE_LIMIT => return_error!(RuntimeError::ResourceLimit(msg)),
                        ET::QUERY_LOGIC => return_error!(RuntimeError::QueryLogic(msg)),
                        ET::NON_EXISTENCE => return_error!(RuntimeError::NonExistence(msg)),
                        ET::OP_FAILED => return_error!(AvailabilityError::OpFailed(msg)),
                        ET::OP_INDETERMINATE => return_error!(AvailabilityError::OpIndeterminate(msg)),
                        ET::USER => return_error!(RuntimeError::User(msg)),
                        ET::PERMISSION_ERROR => return_error!(RuntimeError::Permission(msg)),
                    }
                } else {
                    return_error!(ResponseError::Db(result.r));
                }
            }
            if has_generic_error {
                match respt {
                    RT::CLIENT_ERROR => return_error!(DriverError::Other(msg)),
                    RT::COMPILE_ERROR => return_error!(Error::Compile(msg)),
                    RT::RUNTIME_ERROR => return_error!(ResponseError::Db(result.r)),
                    _ => {/* not an error */},
                }
            }
            // Since this is a successful query let's process the results and send
            // them to the caller
            if let Ok(stati) = from_value::<Vec<WriteStatus>>(result.r.clone()) {
                for v in stati {
                    tx = try_tx!(tx.send(Ok(ResponseValue::Write(v))).wait());
                }
            } else if let Ok(data) = from_value::<Vec<T>>(result.r.clone()) {
                for v in data {
                    tx = try_tx!(tx.send(Ok(ResponseValue::Read(v))).wait());
                }
            } else {
                // Send unexpected query response
                // This is not an error according to the database
                // but the caller wasn't expecting such a response
                // so we just return it raw.
                tx = try_tx!(tx.send(Ok(ResponseValue::Raw(result.r.clone()))).wait());
            }
            // Return response type so we know if we need to retrieve more data
            (tx, true, retry, Ok(respt))
        },
        // We failed to read the server's response so we will
        // try again as long as we haven't used up all our allowed retries.
        Err(error) => {
            retry = true;
            return_error!(error);
        },
    }
}

fn wrap_command<T>(command: TT,
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

fn wrap_query(query_type: QT,
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

fn write_query(query: &str, conn: &mut Connection) -> Result<()> {
    let query = query.as_bytes();
    let token = conn.token;
    if let Err(error) = conn.stream.write_u64::<LittleEndian>(token) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.write_u32::<LittleEndian>(query.len() as u32) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.write_all(query) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.flush() {
        conn.broken = true;
        return error!(error);
    }
    Ok(())
}

fn read_query(conn: &mut Connection) -> Result<Vec<u8>> {
    let _ = match conn.stream.read_u64::<LittleEndian>() {
        Ok(token) => token,
        Err(error) => {
            conn.broken = true;
            return error!(error);
        }
    };
    let len = match conn.stream.read_u32::<LittleEndian>() {
        Ok(len) => len,
        Err(error) => {
            conn.broken = true;
            return error!(error);
        }
    };
    let mut resp = vec![0u8; len as usize];
    if let Err(error) = conn.stream.read_exact(&mut resp) {
        conn.broken = true;
        return error!(error);
    }
    Ok(resp)
}

fn wrap_arrays(mut val: Value) -> Value {
    if val.is_array() {
        let mut array = Vec::with_capacity(2);
        array.push(Value::I64(TT::MAKE_ARRAY.value() as i64));
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
