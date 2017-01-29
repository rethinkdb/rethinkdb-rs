use std::net::TcpStream;
use std::str;
use std::io::{Write, BufRead, Read};
use std::sync::mpsc::SyncSender;

use errors::*;
use Result;
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use bufstream::BufStream;
use protobuf::ProtobufEnum;
use scram::{ClientFirst, ServerFirst, ServerFinal};
use uuid::Uuid;
use serde::Deserialize;
use serde_json::{
    Value,
    from_str, from_slice, from_value,
    to_vec,
};
use ql2::proto::{
    VersionDummy_Version as Version,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
     success: bool,
     min_protocol_version: usize,
     max_protocol_version: usize,
     server_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: i32,
    authentication_method: String,
    authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
     success: bool,
     authentication: Option<String>,
     error_code: Option<usize>,
     error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
     authentication: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqlResponse {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

/// Status returned by a write command
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteStatus {
    inserted: u32,
    replaced: u32,
    unchanged: u32,
    skipped: u32,
    deleted: u32,
    errors: u32,
    first_error: Option<String>,
    generated_keys: Option<Vec<Uuid>>,
    warnings: Option<Vec<String>>,
    changes: Option<Value>,
}

macro_rules! error {
    ($e:expr) => {{
        let error = Error::from($e);
        Err(error)
    }}
}

/// Response value
#[derive(Debug, Clone)]
pub enum ResponseValue<T: Deserialize> {
    Write(WriteStatus),
    Read(T),
    Raw(Value),
    None,
}

/// Connection Options
///
/// Implements methods for configuring details to connect to database servers.
#[derive(Debug, Clone)]
pub struct ConnectionOpts {
    servers: Vec<&'static str>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    tls: Option<TlsCfg>,
}

impl ConnectionOpts {
    pub fn servers(&self) -> &Vec<&'static str> {
        &self.servers
    }

    pub fn set_servers(&mut self, servers: Vec<&'static str>) -> &mut Self {
        self.servers = servers;
        self
    }

    pub fn db(&self) -> &'static str {
        self.db
    }

    pub fn set_db(&mut self, db: &'static str) -> &mut Self {
        self.db = db;
        self
    }

    pub fn user(&self) -> &'static str {
        self.user
    }

    pub fn set_user(&mut self, user: &'static str) -> &mut Self {
        self.user = user;
        self
    }

    pub fn password(&self) -> &'static str {
        self.password
    }

    pub fn set_password(&mut self, password: &'static str) -> &mut Self {
        self.password = password;
        self
    }

    pub fn retries(&self) -> u8 {
        self.retries
    }

    pub fn set_retries(&mut self, retries: u8) -> &mut Self {
        self.retries = retries;
        self
    }

    pub fn tls(&self) -> &Option<TlsCfg> {
        &self.tls
    }

    pub fn set_tls(&mut self, tls: Option<TlsCfg>) -> &mut Self {
        self.tls = tls;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TlsCfg {
    ca_certs: &'static str,
}

impl Default for ConnectionOpts {
    fn default() -> ConnectionOpts {
        ConnectionOpts {
            servers: vec!["localhost:28015"],
            db: "test",
            user: "admin",
            password: "",
            retries: 5,
            tls: None,
        }
    }
}

/// A connection to a RethinkDB database.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    token: u64,
    broken: bool,
}

impl Connection {
    pub fn new(server: &'static str, opts: &ConnectionOpts) -> Result<Connection> {
        let mut conn = Connection {
            stream: try!(TcpStream::connect(server)),
            token: 0,
            broken: false,
        };
        let _ = try!(conn.handshake(opts));
        Ok(conn)
    }

    fn handshake(&mut self, opts: &ConnectionOpts) -> Result<()> {
        // Send desired version to the server
        let _ = try!(self.stream
                     .write_u32::<LittleEndian>(Version::V1_0 as u32));
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

    pub fn is_valid(&mut self) -> Result<()> {
        self.incr_token();
        let query = wrap_query(QueryType::START, Some(String::from("1")), None);
        write_query(self, &query)?;
        let resp = read_query(self)?;
        let resp: ReqlResponse = from_slice(&resp[..])?;
        if let Some(respt) = ResponseType::from_i32(resp.t) {
            if let ResponseType::SUCCESS_ATOM = respt {
                let val: Vec<i32> = from_value(resp.r.clone())?;
                if val == [1] {
                    return Ok(());
                }
            }
        }
        let msg = format!("Unexpected response from server: {:?}", resp);
        error!(ConnectionError::Other(msg))
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

fn client_first(opts: &ConnectionOpts) -> Result<(ServerFirst, Vec<u8>)> {
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

impl ReqlConnection for Connection {
    fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    fn incr_token(&mut self) -> &mut Self {
        self.token += 1;
        self
    }

    fn token(&self) -> u64 {
        self.token
    }

    fn set_broken(&mut self, b: bool) -> &mut Self {
        self.broken = b;
        self
    }

    fn broken(&self) -> bool {
        self.broken
    }
}

pub trait ReqlConnection {
    fn stream(&mut self) -> &mut TcpStream;
    fn incr_token(&mut self) -> &mut Self;
    fn token(&self) -> u64;
    fn set_broken(&mut self, b: bool) -> &mut Self;
    fn broken(&self) -> bool;
}

pub trait Session where Self::Connection: ReqlConnection
{
    type Connection;

    fn get(&self) -> Result<Self::Connection>;
}

pub struct Request<T: Deserialize, S: Session> {
    pool: S,
    conn: S::Connection,
    retry: bool,
    write: bool,
    tx: SyncSender<Result<ResponseValue<T>>>,
}

impl<T, S> Request<T, S>
    where S: Session,
          S::Connection: ReqlConnection,
          T: Deserialize + Send,
{
    pub fn new(pool: S, tx: SyncSender<Result<ResponseValue<T>>>) -> Result<Request<T, S>>
    {
        let conn = pool.get()?;
        Ok(Request{
            pool: pool,
            conn: conn,
            retry: false,
            write: true,
            tx: tx,
        })
    }

    pub fn submit(&mut self, cfg: &ConnectionOpts, commands: String, opts: Option<String>) -> Result<()>
    {
        let mut query = wrap_query(QueryType::START, Some(commands), opts);
        // Try sending the query
        {
            let mut i = 0;
            let mut connect = false;
            while i < cfg.retries {
                // Open a new connection if necessary
                if connect {
                    drop(&mut self.conn);
                    self.conn = match self.pool.get() {
                        Ok(c) => c,
                        Err(error) => {
                            if i == cfg.retries - 1 {
                                return error!(error);
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    };
                }
                // Submit the query if necessary
                if self.write {
                    if let Err(error) = self.write(&query) {
                        connect = true;
                        if i == cfg.retries - 1 {
                            return error!(error);
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                    connect = false;
                }
                // Handle the response
                if let Err(error) = self.process(&mut query) {
                    if i == cfg.retries - 1 || !self.retry {
                        return error!(error);
                    }
                    i += 1;
                    continue;
                }
                break;
            }
        }
        Ok(())
    }

    pub fn process(&mut self, query: &mut String) -> Result<()>
    {
        self.retry = false;
        self.write = false;
        match self.handle() {
            Ok(t) => {
                match t {
                    Some(ResponseType::SUCCESS_PARTIAL) => {
                        *query = wrap_query(QueryType::CONTINUE, None, None);
                        if let Err(error) = self.write(query) {
                            self.write = true;
                            self.retry = true;
                            return error!(error);
                        }
                        self.process(query)?;
                    },

                    Some(_)  => {/* we are done */},

                    None => {
                        let msg = String::from("Request::handle() unexpectedly returned None");
                        return error!(DriverError::Other(msg));
                    },
                }
            }
            Err(error) => {
                if let Error::Runtime(RuntimeError::Availability(AvailabilityError::OpFailed(ref msg))) = error {
                    if msg.starts_with("Cannot perform write: primary replica for shard") {
                        self.write = true;
                        self.retry = true;
                    }
                }
                return error!(error);
            }
        }
        Ok(())
    }

    pub fn handle(&mut self) -> Result<Option<ResponseType>>
    {
        self.retry = false;
        match self.read() {
            Ok(resp) => {
                let result: ReqlResponse = from_slice(&resp[..])?;
                let respt: ResponseType;
                if let Some(t) = ResponseType::from_i32(result.t) {
                    respt = t;
                } else {
                    let msg = format!("Unsupported response type ({}), returned by the database.", result.t);
                    return error!(DriverError::Other(msg));
                }
                // If the database says this response is an error convert the error 
                // message to our native one.
                let has_generic_error = match respt {
                    ResponseType::CLIENT_ERROR | ResponseType::COMPILE_ERROR | ResponseType::RUNTIME_ERROR => true,
                    _ => false,
                };
                let mut msg = String::new();
                if result.e.is_some() || has_generic_error {
                    msg = if let Value::Array(error) = result.r.clone() {
                        if error.len() == 1 {
                            if let Some(Value::String(msg)) = error.into_iter().next() {
                                msg
                            } else {
                                return error!(ResponseError::Db(result.r));
                            }
                        } else {
                            return error!(ResponseError::Db(result.r));
                        }
                    } else {
                        return error!(ResponseError::Db(result.r));
                    };
                }
                if let Some(e) = result.e {
                    if let Some(error) = ErrorType::from_i32(e) {
                        match error {
                            ErrorType::INTERNAL => return error!(RuntimeError::Internal(msg)),
                            ErrorType::RESOURCE_LIMIT => return error!(RuntimeError::ResourceLimit(msg)),
                            ErrorType::QUERY_LOGIC => return error!(RuntimeError::QueryLogic(msg)),
                            ErrorType::NON_EXISTENCE => return error!(RuntimeError::NonExistence(msg)),
                            ErrorType::OP_FAILED => return error!(AvailabilityError::OpFailed(msg)),
                            ErrorType::OP_INDETERMINATE => return error!(AvailabilityError::OpIndeterminate(msg)),
                            ErrorType::USER => return error!(RuntimeError::User(msg)),
                            ErrorType::PERMISSION_ERROR => return error!(RuntimeError::Permission(msg)),
                        }
                    } else {
                        return error!(ResponseError::Db(result.r));
                    }
                }
                if has_generic_error {
                    match respt {
                        ResponseType::CLIENT_ERROR => return error!(DriverError::Other(msg)),
                        ResponseType::COMPILE_ERROR => return error!(Error::Compile(msg)),
                        ResponseType::RUNTIME_ERROR => return error!(ResponseError::Db(result.r)),
                        _ => {/* not an error */},
                    }
                }
                // Since this is a successful query let's process the results and send
                // them to the caller
                if let Ok(stati) = from_value::<Vec<WriteStatus>>(result.r.clone()) {
                    for v in stati {
                        let tx = self.tx.clone();
                        tx.send(Ok(ResponseValue::Write(v)))?;
                    }
                } else if let Ok(data) = from_value::<Vec<T>>(result.r.clone()) {
                    for v in data {
                        let tx = self.tx.clone();
                        tx.send(Ok(ResponseValue::Read(v)))?;
                    }
                }
                // Send unexpected query responses
                // This is not an error according to the database
                // but the caller wasn't expecting such a response
                // so we just return it raw.
                else if let Ok(data) = from_value::<Vec<Value>>(result.r.clone()) {
                    for v in data {
                        let tx = self.tx.clone();
                        match v {
                            Value::Null => {
                                tx.send(Ok(ResponseValue::None))?;
                            }
                            value => {
                                tx.send(Ok(ResponseValue::Raw(value)))?;
                            }
                        }
                    }
                } else {
                    let tx = self.tx.clone();
                    tx.send(Ok(ResponseValue::Raw(result.r.clone())))?;
                }
                // Return response type so we know if we need to retrieve more data
                Ok(Some(respt))
            },
            // We failed to read the server's response so we will
            // try again as long as we haven't used up all our allowed retries.
            Err(error) => {
                self.retry = true;
                return error!(error);
            },
        }
    }

    pub fn write(&mut self, query: &str) -> Result<()> {
        write_query(&mut self.conn, query)
    }

    pub fn read(&mut self) -> Result<Vec<u8>> {
        read_query(&mut self.conn)
    }
}

pub fn write_query<C>(conn: &mut C, query: &str) -> Result<()>
    where C: ReqlConnection
{
    let query = query.as_bytes();
    let token = conn.token();
    if let Err(error) = conn.stream().write_u64::<LittleEndian>(token) {
        conn.set_broken(true);
        return error!(error);
    }
    if let Err(error) = conn.stream().write_u32::<LittleEndian>(query.len() as u32) {
        conn.set_broken(true);
        return error!(error);
    }
    if let Err(error) = conn.stream().write_all(query) {
        conn.set_broken(true);
        return error!(error);
    }
    if let Err(error) = conn.stream().flush() {
        conn.set_broken(true);
        return error!(error);
    }
    Ok(())
}

pub fn read_query<C>(conn: &mut C) -> Result<Vec<u8>>
    where C: ReqlConnection
{
    let _ = match conn.stream().read_u64::<LittleEndian>() {
        Ok(token) => token,
        Err(error) => {
            conn.set_broken(true);
            return error!(error);
        }
    };
    let len = match conn.stream().read_u32::<LittleEndian>() {
        Ok(len) => len,
        Err(error) => {
            conn.set_broken(true);
            return error!(error);
        }
    };
    let mut resp = vec![0u8; len as usize];
    if let Err(error) = conn.stream().read_exact(&mut resp) {
        conn.set_broken(true);
        return error!(error);
    }
    Ok(resp)
}

pub fn wrap_query(query_type: QueryType,
              query: Option<String>,
              options: Option<String>)
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
