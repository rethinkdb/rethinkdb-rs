use super::connect::DEFAULT_DB;
use super::StaticString;
use crate::cmd::{Durability, ReadMode};
use crate::proto::Payload;
use crate::{err, Connection, Query, Result, Session};
use async_stream::try_stream;
use futures::io::{AsyncReadExt, AsyncWriteExt};
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
    pub(crate) r: Value,
    b: Option<Vec<Frame>>,
    p: Option<Value>,
    n: Option<Vec<i32>>,
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
    pub noreply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<Db>,
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn noreply(mut self, noreply: bool) -> Self {
        self.noreply = Some(noreply);
        self
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

impl Session {
    fn default_db(&self) -> Options {
        if self.db == DEFAULT_DB {
            Options::new()
        } else {
            Options::new().db(&self.db)
        }
    }
}

impl Options {
    fn default_db(self, session: &Session) -> Options {
        if self.db.is_none() && session.db != DEFAULT_DB {
            return self.db(&session.db);
        }
        self
    }
}

pub trait Arg<'a> {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)>;
}

impl<'a> Arg<'a> for &'a Session {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        let conn = self.connection()?;
        let opts = self.default_db();
        Ok((conn, opts))
    }
}

impl<'a> Arg<'a> for Connection<'a> {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        let opts = self.session.default_db();
        Ok((self, opts))
    }
}

impl<'a> Arg<'a> for (&'a Session, Options) {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        let (session, options) = self;
        let conn = session.connection()?;
        let opts = options.default_db(session);
        Ok((conn, opts))
    }
}

impl<'a> Arg<'a> for (Connection<'a>, Options) {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        let (conn, options) = self;
        let opts = options.default_db(&conn.session);
        Ok((conn, opts))
    }
}

impl<'a> Arg<'a> for &'a mut Session {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        self.connection()?.into_run_opts()
    }
}

impl<'a> Arg<'a> for (&'a mut Session, Options) {
    fn into_run_opts(self) -> Result<(Connection<'a>, Options)> {
        let (session, options) = self;
        let conn = session.connection()?;
        (conn, options).into_run_opts()
    }
}

pub(crate) fn new<'a, A, T>(query: Query, arg: A) -> impl Stream<Item = Result<T>>
where
    A: Arg<'a>,
    T: Unpin + DeserializeOwned,
{
    try_stream! {
        let (mut conn, opts) = arg.into_run_opts()?;
        if query.change_feed() {
            conn.session.mark_change_feed();
        }
        let noreply = opts.noreply.unwrap_or_default();
        let mut payload = Payload(QueryType::Start, Some(query), opts);
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
                    payload = Payload(QueryType::Continue, None, Default::default());
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    continue;
                }
                ResponseType::WaitComplete => { break; }
                typ => {
                    Err(response_error(typ, resp.e, resp))?;
                    break;
                }
            }
        }
    }
}

impl Payload {
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

impl Connection<'_> {
    fn send_response(&self, db_token: u64, resp: Result<(ResponseType, Response)>) {
        if let Some(tx) = self.session.channels.get(&db_token) {
            if let Err(error) = tx.unbounded_send(resp) {
                if error.is_disconnected() {
                    self.session.channels.remove(&db_token);
                }
            }
        }
    }

    pub(crate) async fn request(
        &mut self,
        query: &Payload,
        noreply: bool,
    ) -> Result<(ResponseType, Response)> {
        self.submit(query, noreply).await;
        match self.rx.lock().await.next().await {
            Some(resp) => resp,
            None => {
                Err(err::Driver::Other("sender stream terminated prematurely".to_owned()).into())
            }
        }
    }

    async fn submit(&self, query: &Payload, noreply: bool) {
        let mut db_token = self.token;
        let result = self.exec(query, noreply, &mut db_token).await;
        self.send_response(db_token, result);
    }

    async fn exec(
        &self,
        query: &Payload,
        noreply: bool,
        db_token: &mut u64,
    ) -> Result<(ResponseType, Response)> {
        let buf = query.encode(self.token)?;

        let guard = self.session.stream.lock().await;
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
            if token > self.session.token.load(Ordering::SeqCst) {
                self.session.mark_broken();
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
            super::debug(&buf),
        );

        let resp = serde_json::from_slice::<Response>(&buf)?;
        trace!("response successfully parsed; token: {}", self.token,);

        let response_type = ResponseType::from_i32(resp.t)
            .ok_or_else(|| err::Driver::Other(format!("unknown response type `{}`", resp.t)))?;

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
