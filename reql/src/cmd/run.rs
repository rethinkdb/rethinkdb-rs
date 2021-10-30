use super::args::Args;
use super::connect::DEFAULT_DB;
use crate::cmd::{Durability, ReadMode};
use crate::proto::{Payload, Query};
use crate::{err, r, Command, Connection, Result, Session};
use async_stream::try_stream;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::stream::{Stream, StreamExt};
use ql2::query::QueryType;
use ql2::response::{ErrorType, ResponseType};
use reql_macros::CommandOptions;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::str;
use std::sync::atomic::Ordering;
use tracing::trace;

const DATA_SIZE: usize = 4;
const TOKEN_SIZE: usize = 8;
const HEADER_SIZE: usize = DATA_SIZE + TOKEN_SIZE;

#[derive(Deserialize, Debug)]
pub(crate) struct Response {
    t: i32,
    e: Option<i32>,
    pub(crate) r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

impl Response {
    fn new() -> Self {
        Self {
            t: ResponseType::SuccessAtom as i32,
            e: None,
            r: Value::Array(Vec::new()),
            b: None,
            p: None,
            n: None,
        }
    }
}

#[derive(
    Debug, Clone, CommandOptions, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
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
    pub noreply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<Db>,
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

impl Options {
    async fn default_db(self, session: &Session) -> Options {
        let session_db = session.inner.db.lock().await;
        if self.db.is_none() && *session_db != DEFAULT_DB {
            return self.db(&*session_db);
        }
        self
    }
}

pub trait Arg {
    fn into_run_opts(self) -> Result<(Connection, Options)>;
}

impl Arg for &Session {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        let conn = self.connection()?;
        Ok((conn, Default::default()))
    }
}

impl Arg for Connection {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        Ok((self, Default::default()))
    }
}

impl Arg for Args<(&Session, Options)> {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        let Args((session, options)) = self;
        let conn = session.connection()?;
        Ok((conn, options))
    }
}

impl Arg for Args<(Connection, Options)> {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        let Args(arg) = self;
        Ok(arg)
    }
}

impl Arg for &mut Session {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        self.connection()?.into_run_opts()
    }
}

impl Arg for Args<(&mut Session, Options)> {
    fn into_run_opts(self) -> Result<(Connection, Options)> {
        let Args((session, options)) = self;
        let conn = session.connection()?;
        r.args((conn, options)).into_run_opts()
    }
}

pub(crate) fn new<A, T>(query: Command, arg: A) -> impl Stream<Item = Result<T>>
where
    A: Arg,
    T: Unpin + DeserializeOwned,
{
    try_stream! {
        let (mut conn, mut opts) = arg.into_run_opts()?;
        opts = opts.default_db(&conn.session).await;
        let change_feed = query.change_feed();
        if change_feed {
            conn.session.inner.mark_change_feed();
        }
        let noreply = opts.noreply.unwrap_or_default();
        let mut payload = Payload(QueryType::Start, Some(Query(&query)), opts);
        loop {
            let (response_type, resp) = conn.request(&payload, noreply).await?;
            trace!("yielding response; token: {}", conn.token);
            match response_type {
                ResponseType::SuccessAtom | ResponseType::SuccessSequence | ResponseType::ServerInfo => {
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    break;
                }
                ResponseType::SuccessPartial => {
                    if conn.closed() {
                        // reopen so we can use the connection in future
                        conn.set_closed(false);
                        trace!("connection closed; token: {}", conn.token);
                        break;
                    }
                    payload = Payload(QueryType::Continue, None, Default::default());
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    continue;
                }
                ResponseType::WaitComplete => { break; }
                typ => {
                    let msg = error_message(resp.r)?;
                    match typ {
                        // This feed has been closed by conn.close().
                        ResponseType::ClientError if change_feed && msg.contains("not in stream cache") => { break; }
                        _ => Err(response_error(typ, resp.e, msg))?,
                    }
                }
            }
        }
    }
}

impl Payload<'_> {
    fn encode(&self, token: u64) -> Result<Vec<u8>> {
        let bytes = self.to_bytes()?;
        let data_len = bytes.len();
        let mut buf = Vec::with_capacity(HEADER_SIZE + data_len);
        buf.extend_from_slice(&token.to_le_bytes());
        buf.extend_from_slice(&(data_len as u32).to_le_bytes());
        buf.extend_from_slice(&bytes);
        Ok(buf)
    }
}

impl Connection {
    fn send_response(&self, db_token: u64, resp: Result<(ResponseType, Response)>) {
        if let Some(tx) = self.session.inner.channels.get(&db_token) {
            if let Err(error) = tx.unbounded_send(resp) {
                if error.is_disconnected() {
                    self.session.inner.channels.remove(&db_token);
                }
            }
        }
    }

    pub(crate) async fn request<'a>(
        &mut self,
        query: &'a Payload<'a>,
        noreply: bool,
    ) -> Result<(ResponseType, Response)> {
        self.submit(query, noreply).await;
        match self.rx.lock().await.next().await {
            Some(resp) => resp,
            None => Ok((ResponseType::SuccessAtom, Response::new())),
        }
    }

    async fn submit<'a>(&self, query: &'a Payload<'a>, noreply: bool) {
        let mut db_token = self.token;
        let result = self.exec(query, noreply, &mut db_token).await;
        self.send_response(db_token, result);
    }

    async fn exec<'a>(
        &self,
        query: &'a Payload<'a>,
        noreply: bool,
        db_token: &mut u64,
    ) -> Result<(ResponseType, Response)> {
        let buf = query.encode(self.token)?;

        let guard = self.session.inner.stream.lock().await;
        let mut stream = guard.clone();

        trace!("sending query; token: {}, payload: {}", self.token, query);
        stream.write_all(&buf).await?;
        trace!("query sent; token: {}", self.token);

        if noreply {
            return Ok((ResponseType::SuccessAtom, Response::new()));
        }

        trace!("reading header; token: {}", self.token);
        let mut header = [0u8; HEADER_SIZE];
        stream.read_exact(&mut header).await?;

        let mut buf = [0u8; TOKEN_SIZE];
        buf.copy_from_slice(&header[..TOKEN_SIZE]);
        *db_token = {
            let token = u64::from_le_bytes(buf);
            trace!("db_token: {}", token);
            if token > self.session.inner.token.load(Ordering::SeqCst) {
                self.session.inner.mark_broken();
                return Err(err::Driver::ConnectionBroken.into());
            }
            token
        };

        let mut buf = [0u8; DATA_SIZE];
        buf.copy_from_slice(&header[TOKEN_SIZE..]);
        let len = u32::from_le_bytes(buf) as usize;
        trace!(
            "header read; token: {}, db_token: {}, response_len: {}",
            self.token,
            db_token,
            len
        );

        trace!("reading body; token: {}", self.token);
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf).await?;

        trace!(
            "body read; token: {}, db_token: {}, body: {}",
            self.token,
            db_token,
            super::bytes_to_string(&buf),
        );

        let resp = serde_json::from_slice::<Response>(&buf)?;
        trace!("response successfully parsed; token: {}", self.token,);

        let response_type = ResponseType::from_i32(resp.t)
            .ok_or_else(|| err::Driver::Other(format!("unknown response type `{}`", resp.t)))?;

        if let Some(error_type) = resp.e {
            let msg = error_message(resp.r)?;
            return Err(response_error(response_type, Some(error_type), msg));
        }

        Ok((response_type, resp))
    }
}

fn error_message(response: Value) -> Result<String> {
    let messages = serde_json::from_value::<Vec<String>>(response)?;
    Ok(messages.join(" "))
}

fn response_error(response_type: ResponseType, error_type: Option<i32>, msg: String) -> err::Error {
    match response_type {
        ResponseType::ClientError => err::Driver::Other(msg).into(),
        ResponseType::CompileError => err::Error::Compile(msg),
        ResponseType::RuntimeError => match error_type
            .map(ErrorType::from_i32)
            .ok_or_else(|| err::Driver::Other(format!("unexpected runtime error: {}", msg)))
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
            _ => err::Driver::Other(format!("unexpected runtime error: {}", msg)).into(),
        },
        _ => err::Driver::Other(format!("unexpected response: {}", msg)).into(),
    }
}
