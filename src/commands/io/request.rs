use std::error::Error as StdError;

use errors::*;
use {Session, ReqlResponse, WriteStatus, Result, ResponseValue};
use super::{Request, wrap_query, write_query, read_query};

use serde::Deserialize;
use futures::{Future, Sink};
use protobuf::ProtobufEnum;
use types::Encode;
use ql2::proto::{
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};
use serde_json::{
    Value,
    from_slice, from_value,
};

impl<T: Deserialize + Send + 'static> Request<T> {
    pub fn submit(mut self) -> Result<()> {
        let mut conn = self.pool.get()?;
        let commands = self.term.encode();
        debug!(conn.logger, "{}", commands);
        let opts = self.opts.encode();
        debug!(conn.logger, "{}", opts);
        let mut query = wrap_query(QueryType::START, Some(commands), Some(opts));
        debug!(conn.logger, "{}", query);
        // Try sending the query
        debug!(conn.logger, "submiting to server");
        {
            let mut i = 0;
            let mut connect = false;
            while i < self.cfg.opts.retries {
                // Open a new connection if necessary
                if connect {
                    drop(&mut conn);
                    conn = match self.pool.get() {
                        Ok(c) => c,
                        Err(error) => {
                            if i == self.cfg.opts.retries - 1 {
                                return Err(error)?;
                            } else {
                                i += 1;
                                continue;
                            }
                        }
                    };
                }
                // Submit the query if necessary
                if self.write {
                    if let Err(error) = write_query(&mut conn, &query) {
                        connect = true;
                        if i == self.cfg.opts.retries - 1 {
                            return Err(error)?;
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                    connect = false;
                }
                // Handle the response
                if let Err(error) = self.process(&mut conn, &mut query) {
                    if i == self.cfg.opts.retries - 1 || !self.retry {
                        return Err(error)?;
                    }
                    i += 1;
                    continue;
                }
                break;
            }
        }
        Ok(())
    }

    pub fn process(&mut self, conn: &mut Session, query: &mut String) -> Result<()>
    {
        self.retry = false;
        self.write = false;
        match self.handle(conn) {
            Ok(t) => {
                match t {
                    Some(ResponseType::SUCCESS_PARTIAL) => {
                        *query = wrap_query(QueryType::CONTINUE, None, None);
                        if let Err(error) = write_query(conn, query) {
                            self.write = true;
                            self.retry = true;
                            return Err(error)?;
                        }
                        self.process(conn, query)?;
                    },

                    Some(_)  => {/* we are done */},

                    None => {
                        let msg = String::from("Request::handle() unexpectedly returned None");
                        return Err(DriverError::Other(msg))?;
                    },
                }
            }
            Err(error) => {
                if error.description().starts_with("Cannot perform write: primary replica for shard") {
                    self.write = true;
                    self.retry = true;
                }
                return Err(error)?;
            }
        }
        Ok(())
    }

    pub fn handle(&mut self, conn: &mut Session) -> Result<Option<ResponseType>>
    {
        self.retry = false;
        match read_query(conn) {
            Ok(resp) => {
                let result: ReqlResponse = from_slice(&resp[..])?;
                let respt: ResponseType;
                if let Some(t) = ResponseType::from_i32(result.t) {
                    respt = t;
                } else {
                    let msg = format!("Unsupported response type ({}), returned by the database.", result.t);
                    return Err(DriverError::Other(msg))?;
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
                                return Err(ResponseError::Db(result.r))?;
                            }
                        } else {
                            return Err(ResponseError::Db(result.r))?;
                        }
                    } else {
                        return Err(ResponseError::Db(result.r))?;
                    };
                }
                if let Some(e) = result.e {
                    if let Some(error) = ErrorType::from_i32(e) {
                        match error {
                            ErrorType::INTERNAL => return Err(RuntimeError::Internal(msg))?,
                            ErrorType::RESOURCE_LIMIT => return Err(RuntimeError::ResourceLimit(msg))?,
                            ErrorType::QUERY_LOGIC => return Err(RuntimeError::QueryLogic(msg))?,
                            ErrorType::NON_EXISTENCE => return Err(RuntimeError::NonExistence(msg))?,
                            ErrorType::OP_FAILED => return Err(AvailabilityError::OpFailed(msg))?,
                            ErrorType::OP_INDETERMINATE => return Err(AvailabilityError::OpIndeterminate(msg))?,
                            ErrorType::USER => return Err(RuntimeError::User(msg))?,
                            ErrorType::PERMISSION_ERROR => return Err(RuntimeError::Permission(msg))?,
                        }
                    } else {
                        return Err(ResponseError::Db(result.r))?;
                    }
                }
                if has_generic_error {
                    match respt {
                        ResponseType::CLIENT_ERROR => return Err(DriverError::Other(msg))?,
                        ResponseType::COMPILE_ERROR => return Err(Error::Compile(msg))?,
                        ResponseType::RUNTIME_ERROR => return Err(ResponseError::Db(result.r))?,
                        _ => {/* not an error */},
                    }
                }
                // Since this is a successful query let's process the results and send
                // them to the caller
                if let Ok(stati) = from_value::<Vec<WriteStatus>>(result.r.clone()) {
                    for v in stati {
                        let tx = self.tx.clone();
                        tx.send(Ok(ResponseValue::Write(v))).wait()?;
                    }
                } else if let Ok(data) = from_value::<Vec<T>>(result.r.clone()) {
                    for v in data {
                        let tx = self.tx.clone();
                        tx.send(Ok(ResponseValue::Read(v))).wait()?;
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
                                tx.send(Ok(ResponseValue::None)).wait()?;
                            }
                            value => {
                                tx.send(Ok(ResponseValue::Raw(value))).wait()?;
                            }
                        }
                    }
                } else {
                    let tx = self.tx.clone();
                    tx.send(Ok(ResponseValue::Raw(result.r.clone()))).wait()?;
                }
                // Return response type so we know if we need to retrieve more data
                Ok(Some(respt))
            },
            // We failed to read the server's response so we will
            // try again as long as we haven't used up all our allowed retries.
            Err(error) => {
                self.retry = true;
                return Err(error)?;
            },
        }
    }
}
