use std::net::TcpStream;
use std::str;
use std::io::{Write, BufRead, Read};

use {Session, Result, Opts};
use errors::*;
use super::io_error;
use reql_io::scram::{ClientFirst, ServerFirst, ServerFinal};
use reql_io::bufstream::BufStream;
use reql_io::byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use reql_io::uuid::Uuid;
use ql2::proto::{
    VersionDummy_Version as Version,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
};
use protobuf::ProtobufEnum;
use serde_json::{
    Value,
    from_str, from_slice, from_value,
    to_vec,
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

impl Session {
    pub fn handshake(&mut self, opts: &Opts) -> Result<()> {
        // Send desired version to the server
        let _ = self.stream
                     .write_u32::<LittleEndian>(Version::V1_0 as u32)?;
        parse_server_version(&self.stream)?;

        // Send client first message
        let (scram, msg) = client_first(opts)?;
        let _ = self.stream.write_all(&msg[..])?;

        // Send client final message
        let (scram, msg) = client_final(scram, &self.stream)?;
        let _ = self.stream.write_all(&msg[..])?;

        // Validate final server response and flush the buffer
        parse_server_final(scram, &self.stream)?;
        let _ = self.stream.flush()?;

        Ok(())
    }

    pub fn is_valid(&mut self) -> Result<()> {
        self.id += 1;
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
        Err(io_error(msg))?
    }
}

fn parse_server_version(stream: &TcpStream) -> Result<()> {
    let resp = parse_server_response(stream)?;
    let info: ServerInfo = from_str(&resp)?;
    if !info.success {
        return Err(io_error(resp.to_string()))?;
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
    let _ = buf.read_until(b'\0', &mut resp)?;

    let _ = resp.pop();

    if resp.is_empty() {
        let msg = String::from("unable to connect for an unknown reason");
        return Err(io_error(msg))?;
    };

    let resp = str::from_utf8(&resp)?.to_string();
    // If it's not a JSON object it's an error
    if !resp.starts_with("{") {
        return Err(io_error(resp))?;
    };
    Ok(resp)
}

fn client_first(opts: &Opts) -> Result<(ServerFirst, Vec<u8>)> {
    let scram = ClientFirst::new(&opts.user, &opts.password, None)?;
    let (scram, client_first) = scram.client_first();

    let ar = AuthRequest {
        protocol_version: 0,
        authentication_method: String::from("SCRAM-SHA-256"),
        authentication: client_first,
    };
    let mut msg = to_vec(&ar)?;
    msg.push(b'\0');
    Ok((scram, msg))
}

fn client_final(scram: ServerFirst, stream: &TcpStream) -> Result<(ServerFinal, Vec<u8>)> {
    let resp = parse_server_response(stream)?;
    let info: AuthResponse = from_str(&resp)?;

    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return Err(DriverError::Auth(err))?;
        } else {
            return Err(io_error(err))?;
        }
    };

    if let Some(auth) = info.authentication {
        let scram = scram.handle_server_first(&auth).unwrap();
        let (scram, client_final) = scram.client_final();
        let auth = AuthConfirmation { authentication: client_final };
        let mut msg = to_vec(&auth)?;
        msg.push(b'\0');
        Ok((scram, msg))
    } else {
        Err(io_error(String::from("Server did not send authentication \
                                                            info.")))?
    }
}

fn parse_server_final(scram: ServerFinal, stream: &TcpStream) -> Result<()> {
    let resp = parse_server_response(stream)?;
    let info: AuthResponse = from_str(&resp)?;
    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return Err(DriverError::Auth(err))?;
        } else {
            return Err(io_error(err))?;
        }
    };
    if let Some(auth) = info.authentication {
        if let Err(err) = scram.handle_server_final(&auth) {
            return Err(io_error(err))?;
        }
    }
    Ok(())
}

fn write_query(conn: &mut Session, query: &str) -> Result<()> {
    let query = query.as_bytes();
    let token = conn.id;
    if let Err(error) = conn.stream.write_u64::<LittleEndian>(token) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.write_u32::<LittleEndian>(query.len() as u32) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.write_all(query) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.flush() {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    Ok(())
}

fn read_query(conn: &mut Session) -> Result<Vec<u8>> {
    let _ = match conn.stream.read_u64::<LittleEndian>() {
        Ok(token) => token,
        Err(error) => {
            conn.broken = true;
            return Err(io_error(error))?;
        }
    };
    let len = match conn.stream.read_u32::<LittleEndian>() {
        Ok(len) => len,
        Err(error) => {
            conn.broken = true;
            return Err(io_error(error))?;
        }
    };
    let mut resp = vec![0u8; len as usize];
    if let Err(error) = conn.stream.read_exact(&mut resp) {
        conn.broken = true;
        return Err(io_error(error))?;
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
