use crate::proto::Payload;
use crate::{err, Connection, Query, Result, TcpStream};
use async_stream::try_stream;
use futures::channel::mpsc;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use futures::sink::SinkExt;
use futures::stream::{Stream, StreamExt};
use log::trace;
use ql2::query::QueryType;
use ql2::response::{ErrorType, ResponseType};
use ql2::Frame;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str;
use std::sync::atomic::Ordering;

const DATA_SIZE: usize = 4;
const TOKEN_SIZE: usize = 8;
const HEADER_SIZE: usize = DATA_SIZE + TOKEN_SIZE;

#[derive(Deserialize, Debug)]
pub struct Response {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Vec<Frame>>,
    p: Option<Value>,
    n: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Options<'a> {
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
    pub db: Option<Db<'a>>,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Native,
    Raw,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Db<'a>(pub &'a str);

pub trait Arg<'a, T> {
    fn arg(self) -> (&'a Connection<'a, T>, Option<Options<'a>>);
}

impl<'a, T> Arg<'a, T> for &'a Connection<'a, T> {
    fn arg(self) -> (&'a Connection<'a, T>, Option<Options<'a>>) {
        (self, None)
    }
}

impl<'a, T> Arg<'a, T> for (&'a Connection<'a, T>, Options<'a>) {
    fn arg(self) -> (&'a Connection<'a, T>, Option<Options<'a>>) {
        (self.0, Some(self.1))
    }
}

pub(crate) fn new<'a, S, A, T>(query: Query, arg: A) -> impl Stream<Item = Result<T>>
where
    S: TcpStream<'a>,
    &'a S: AsyncRead + AsyncWrite,
    A: Arg<'a, S>,
    T: Unpin + DeserializeOwned,
{
    try_stream! {
        let (conn, opts) = arg.arg();
        conn.broken()?;
        conn.change_feed()?;
        if query.change_feed {
            conn.mark_change_feed();
        }
        let token = conn.token();
        let (tx, mut rx) = mpsc::channel(conn.buffer);
        conn.senders.insert(token, tx);
        let payload = Payload(QueryType::Start, Some(query), opts);
        trace!("sending query; token: {}, payload: {}", token, payload);
        conn.write(token, &payload).await?;
        trace!("query sent; token: {}", token);
        loop {
            trace!("receiving response; token: {}", token);
            let resp = conn.read(token, &mut rx).await?;
            let response_type = ResponseType::from_i32(resp.t)
                .ok_or_else(|| err::Client::Other(format!("uknown response type `{}`", resp.t)))?;
            if let Some(error_type) = resp.e {
                response_error(response_type, Some(error_type), resp)?;
                break;
            }
            match response_type {
                ResponseType::SuccessAtom | ResponseType::SuccessSequence | ResponseType::ServerInfo => {
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    break;
                }
                ResponseType::SuccessPartial => {
                    let payload = Payload(QueryType::Continue, None, None);
                    conn.write(token, &payload).await?;
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    continue;
                }
                // TODO what whould we yield here?
                ResponseType::WaitComplete => { break; }
                typ => {
                    response_error(typ, resp.e, resp)?;
                    break;
                }
            }
        }
        rx.close();
        conn.senders.remove(&token);
        conn.unmark_change_feed();
    }
}

impl<'a, S> Connection<'a, S>
where
    S: TcpStream<'a>,
    &'a S: AsyncRead + AsyncWrite,
{
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

    async fn write(&'a self, token: u64, query: &Payload<'_>) -> Result<()> {
        let bytes = query.to_bytes()?;
        let data_len = bytes.len();
        let mut buf = Vec::with_capacity(HEADER_SIZE + data_len);
        buf.extend_from_slice(&token.to_le_bytes());
        buf.extend_from_slice(&(data_len as u32).to_le_bytes());
        buf.extend_from_slice(&bytes);
        (&mut &self.stream).write_all(&buf).await?;
        Ok(())
    }

    async fn read(
        &'a self,
        token: u64,
        rx: &mut mpsc::Receiver<Result<Response>>,
    ) -> Result<Response> {
        // reading data from RethinkDB is a 2 step process so
        // we have to get a lock to ensure no other process
        // reads the data while we are reading it
        let guard = self.locker.lock().await;
        trace!("reading header; token: {}", token);
        let mut header = [0u8; HEADER_SIZE];
        (&mut &self.stream).read(&mut header).await?;

        let mut buf = [0u8; TOKEN_SIZE];
        buf.copy_from_slice(&header[..TOKEN_SIZE]);
        let db_token = u64::from_le_bytes(buf);

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
        let result = (&mut &self.stream).read(&mut buf).await;

        // we have finished reading so we can drop the lock
        // and let other processes advance
        drop(guard);

        trace!(
            "body {}; token: {}, db_token: {}",
            if result.is_ok() {
                "read"
            } else {
                "reading failed"
            },
            token,
            db_token,
        );

        match (result, db_token == token) {
            (result, true) => {
                result?;
                return Ok(serde_json::from_slice(&buf)?);
            }
            (Ok(_), false) => {
                let response = serde_json::from_slice(&buf).map_err(Into::into);
                self.send_response(db_token, response).await;
            }
            (Err(error), false) => {
                self.send_response(db_token, Err(error.into())).await;
            }
        }

        match rx.next().await {
            Some(resp) => resp,
            None => {
                Err(err::Client::Other("sender stream terminated prematurely".to_owned()).into())
            }
        }
    }

    async fn send_response(&self, db_token: u64, resp: Result<Response>) {
        if let Some(sender) = self.senders.get(&db_token) {
            let mut tx = sender.clone();
            if let Err(error) = tx.send(resp).await {
                if error.is_disconnected() {
                    self.senders.remove(&db_token);
                }
            }
        }
    }
}

fn response_error(
    response_type: ResponseType,
    error_type: Option<i32>,
    resp: Response,
) -> Result<()> {
    let msg = serde_json::from_value::<Vec<String>>(resp.r)?.join(" ");
    Err(match response_type {
        ResponseType::ClientError => err::Client::Other(msg).into(),
        ResponseType::CompileError => err::Error::Compile(msg),
        ResponseType::RuntimeError => match ErrorType::from_i32(
            error_type
                .ok_or_else(|| err::Client::Other(format!("unexpected runtime error: {}", msg)))?,
        ) {
            Some(ErrorType::Internal) => err::Runtime::Internal(msg).into(),
            Some(ErrorType::ResourceLimit) => err::Runtime::ResourceLimit(msg).into(),
            Some(ErrorType::QueryLogic) => err::Runtime::QueryLogic(msg).into(),
            Some(ErrorType::NonExistence) => err::Runtime::NonExistence(msg).into(),
            Some(ErrorType::OpFailed) => err::Availability::OpFailed(msg).into(),
            Some(ErrorType::OpIndeterminate) => err::Availability::OpIndeterminate(msg).into(),
            Some(ErrorType::User) => err::Runtime::User(msg).into(),
            Some(ErrorType::PermissionError) => err::Runtime::Permission(msg).into(),
            _ => err::Client::Other(format!("unexpected runtime error: {}", msg)).into(),
        },
        _ => err::Client::Other(format!("unexpected response: {}", msg)).into(),
    })
}
