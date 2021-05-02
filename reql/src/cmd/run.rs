use super::connect::DEFAULT_DB;
use super::StaticString;
use crate::cmd::{Durability, ReadMode};
use crate::proto::Payload;
use crate::{err, Connection, Query, Result};
use async_stream::try_stream;
use futures::channel::mpsc::{self, UnboundedReceiver};
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::join;
use futures::stream::{Stream, StreamExt};
use log::trace;
use ql2::query::QueryType;
use ql2::response::{ErrorType, ResponseType};
use ql2::Frame;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::str;
use std::sync::atomic::Ordering;

const DATA_SIZE: usize = 4;
const TOKEN_SIZE: usize = 8;
const HEADER_SIZE: usize = DATA_SIZE + TOKEN_SIZE;

#[derive(Deserialize, Debug)]
pub(crate) struct Response {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Vec<Frame>>,
    p: Option<Value>,
    n: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<Db>,
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn db<T: StaticString>(mut self, db: T) -> Self {
        self.db = Some(Db(db.static_string()));
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Native,
    Raw,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Db(pub Cow<'static, str>);

pub trait Arg<'a> {
    fn into(self) -> (&'a Connection, Options);
}

impl<'a> Arg<'a> for &'a Connection {
    fn into(self) -> (&'a Connection, Options) {
        let opts = if self.db == DEFAULT_DB {
            Options::new()
        } else {
            Options::new().db(&self.db)
        };
        (self, opts)
    }
}

impl<'a> Arg<'a> for (&'a Connection, Options) {
    fn into(self) -> (&'a Connection, Options) {
        let (conn, options) = self;
        let opts = if options.db.is_none() && conn.db != DEFAULT_DB {
            options.db(&conn.db)
        } else {
            options
        };
        (conn, opts)
    }
}

pub(crate) fn new<'a, A, T>(query: Query, arg: A) -> impl Stream<Item = Result<T>>
where
    A: Arg<'a>,
    T: Unpin + DeserializeOwned,
{
    try_stream! {
        let (conn, opts) = arg.into();
        conn.broken()?;
        conn.change_feed()?;
        if query.change_feed() {
            conn.mark_change_feed();
        }
        let token = conn.token();
        let (tx, mut rx) = mpsc::unbounded();
        conn.channels.insert(token, tx);
        let mut payload = Payload(QueryType::Start, Some(query), opts);
        loop {
            let (response_type, resp) = conn.request(token, &payload, &mut rx).await?;
            trace!("yielding response; token: {}, response: {}", token, resp.r);
            match response_type {
                ResponseType::SuccessAtom | ResponseType::SuccessSequence | ResponseType::ServerInfo => {
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    break;
                }
                ResponseType::SuccessPartial => {
                    payload = Payload(QueryType::Continue, None, Default::default());
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    continue;
                }
                // TODO what whould we yield here?
                ResponseType::WaitComplete => { break; }
                typ => {
                    Err(response_error(typ, resp.e, resp))?;
                    break;
                }
            }
        }
        rx.close();
        conn.channels.remove(&token);
        conn.unmark_change_feed();
    }
}

impl Connection {
    fn token(&self) -> u64 {
        let token = self
            .token
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1))
            .unwrap();
        if token == u64::MAX {
            self.mark_broken();
        }
        token
    }

    fn send_response(&self, db_token: u64, resp: Result<(ResponseType, Response)>) {
        if let Some(tx) = self.channels.get(&db_token) {
            if let Err(error) = tx.unbounded_send(resp) {
                if error.is_disconnected() {
                    self.channels.remove(&db_token);
                }
            }
        }
    }

    async fn request(
        &self,
        token: u64,
        query: &Payload,
        rx: &mut UnboundedReceiver<Result<(ResponseType, Response)>>,
    ) -> Result<(ResponseType, Response)> {
        let (_, result) = join!(self.submit(token, query), rx.next());
        match result {
            Some(resp) => resp,
            None => {
                Err(err::Client::Other("sender stream terminated prematurely".to_owned()).into())
            }
        }
    }

    async fn submit(&self, token: u64, query: &Payload) {
        let mut db_token = token;
        let result = self.exec(token, query, &mut db_token).await;
        self.send_response(db_token, result);
    }

    async fn exec(
        &self,
        token: u64,
        query: &Payload,
        db_token: &mut u64,
    ) -> Result<(ResponseType, Response)> {
        let bytes = query.to_bytes()?;
        let data_len = bytes.len();
        let mut buf = Vec::with_capacity(HEADER_SIZE + data_len);
        buf.extend_from_slice(&token.to_le_bytes());
        buf.extend_from_slice(&(data_len as u32).to_le_bytes());
        buf.extend_from_slice(&bytes);

        let guard = self.stream.lock().await;
        let mut stream = guard.clone();

        trace!("sending query; token: {}, payload: {}", token, query);
        stream.write_all(&buf).await?;
        trace!("query sent; token: {}", token);

        trace!("reading header; token: {}", token);
        let mut header = [0u8; HEADER_SIZE];
        stream.read_exact(&mut header).await?;

        let mut buf = [0u8; TOKEN_SIZE];
        buf.copy_from_slice(&header[..TOKEN_SIZE]);
        *db_token = {
            let token = u64::from_le_bytes(buf);
            trace!("db_token: {}", token);
            if token > self.token.load(Ordering::SeqCst) {
                self.mark_broken();
                return Err(err::Client::ConnectionBroken.into());
            }
            token
        };

        let mut buf = [0u8; DATA_SIZE];
        buf.copy_from_slice(&header[TOKEN_SIZE..]);
        let len = u32::from_le_bytes(buf) as usize;
        trace!(
            "header read; token: {}, db_token: {}, response_len: {}",
            token,
            db_token,
            len
        );

        trace!("reading body; token: {}", token);
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf).await?;

        trace!(
            "body read; token: {}, db_token: {}, body: {}",
            token,
            db_token,
            super::debug(&buf),
        );

        let resp = serde_json::from_slice::<Response>(&buf)?;
        trace!(
            "response successfully parsed; token: {}, response: {}",
            token,
            resp.r
        );

        let response_type = ResponseType::from_i32(resp.t)
            .ok_or_else(|| err::Client::Other(format!("uknown response type `{}`", resp.t)))?;

        if let Some(error_type) = resp.e {
            return Err(response_error(response_type, Some(error_type), resp));
        }

        Ok((response_type, resp))
    }
}

fn response_error(
    response_type: ResponseType,
    error_type: Option<i32>,
    resp: Response,
) -> err::Error {
    let msg = match serde_json::from_value::<Vec<String>>(resp.r) {
        Ok(messages) => messages.join(" "),
        Err(error) => {
            return error.into();
        }
    };
    match response_type {
        ResponseType::ClientError => err::Client::Other(msg).into(),
        ResponseType::CompileError => err::Error::Compile(msg),
        ResponseType::RuntimeError => match error_type
            .map(ErrorType::from_i32)
            .ok_or_else(|| err::Client::Other(format!("unexpected runtime error: {}", msg)))
        {
            Ok(Some(ErrorType::Internal)) => err::Runtime::Internal(msg).into(),
            Ok(Some(ErrorType::ResourceLimit)) => err::Runtime::ResourceLimit(msg).into(),
            Ok(Some(ErrorType::QueryLogic)) => err::Runtime::QueryLogic(msg).into(),
            Ok(Some(ErrorType::NonExistence)) => err::Runtime::NonExistence(msg).into(),
            Ok(Some(ErrorType::OpFailed)) => err::Availability::OpFailed(msg).into(),
            Ok(Some(ErrorType::OpIndeterminate)) => err::Availability::OpIndeterminate(msg).into(),
            Ok(Some(ErrorType::User)) => err::Runtime::User(msg).into(),
            Ok(Some(ErrorType::PermissionError)) => err::Runtime::Permission(msg).into(),
            Err(error) => error.into(),
            _ => err::Client::Other(format!("unexpected runtime error: {}", msg)).into(),
        },
        _ => err::Client::Other(format!("unexpected response: {}", msg)).into(),
    }
}
