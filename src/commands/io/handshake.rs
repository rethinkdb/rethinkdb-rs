use super::{io_error, read_query, wrap_query, write_query};
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::str;

use crate::{errors::*, Opts, ReqlResponse, Result, Session};
use bufstream::BufStream;
use byteorder::{LittleEndian, WriteBytesExt};
use protobuf::ProtobufEnum;
use ql2::proto::{
    Query_QueryType as QueryType, Response_ResponseType as ResponseType,
    VersionDummy_Version as Version,
};
use scram::client::{ScramClient, ServerFinal};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, from_str, from_value, to_vec};

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

impl Session {
    pub fn handshake(&mut self, opts: &Opts) -> Result<()> {
        // Send desired version to the server
        let _ = self
            .stream
            .write_u32::<LittleEndian>(Version::V1_0 as u32)?;
        parse_server_version(&self.stream)?;

        // Send client first message
        let scram = ScramClient::new(&opts.user, &opts.password, None);
        let (scram, client_first) = scram.client_first();

        let ar = AuthRequest {
            protocol_version: 0,
            authentication_method: String::from("SCRAM-SHA-256"),
            authentication: client_first,
        };
        let mut msg = to_vec(&ar)?;
        msg.push(b'\0');
        let _ = self.stream.write_all(&msg[..])?;

        // Send client final message
        let resp = parse_server_response(&self.stream)?;
        let info: AuthResponse = from_str(&resp)?;

        if !info.success {
            let mut err = resp.to_string();
            if let Some(e) = info.error {
                err = e;
            }
            // If error code is between 10 and 20, this is an auth error
            if let Some(10..=20) = info.error_code {
                return Err(DriverError::Auth(err))?;
            } else {
                return Err(io_error(err))?;
            }
        };

        match info.authentication {
            Some(auth) => {
                let scram = scram.handle_server_first(&auth)?;
                let (scram, client_final) = scram.client_final();
                let auth = AuthConfirmation {
                    authentication: client_final,
                };
                let mut msg = to_vec(&auth)?;
                msg.push(b'\0');
                let _ = self.stream.write_all(&msg[..])?;

                // Validate final server response and flush the buffer
                parse_server_final(scram, &self.stream)?;
                let _ = self.stream.flush()?;

                Ok(())
            }
            None => Err(io_error(String::from(
                "Server did not send authentication \
                                                            info.",
            )))?,
        }
    }

    pub fn is_valid(&mut self) -> Result<()> {
        self.id = self.id.wrapping_add(1);
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

fn parse_server_final(scram: ServerFinal, stream: &TcpStream) -> Result<()> {
    let resp = parse_server_response(stream)?;
    let info: AuthResponse = from_str(&resp)?;
    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10..=20) = info.error_code {
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
