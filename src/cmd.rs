pub mod changes;
pub mod connection;
pub mod db;
pub mod expr;
pub mod insert;
pub mod run;
pub mod table;

use crate::{Query, Result, TcpStream};
use futures::io::{AsyncRead, AsyncWrite};
use futures::Stream;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str;

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

fn debug(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}

impl<'a> Query {
    pub fn table<T>(self, arg: T) -> Query
    where
        T: table::Arg,
    {
        table::new(Some(self), arg.arg())
    }

    pub fn insert<T>(self, arg: T) -> Query
    where
        T: insert::Arg,
    {
        insert::new(self, arg.arg())
    }

    pub fn changes<T>(self, arg: T) -> Query
    where
        T: changes::Arg,
    {
        changes::new(self, arg.arg())
    }

    pub fn run<S, A, T>(self, arg: A) -> impl Stream<Item = Result<T>>
    where
        S: TcpStream<'a>,
        &'a S: AsyncRead + AsyncWrite,
        A: run::Arg<'a, S>,
        T: Unpin + DeserializeOwned,
    {
        Box::pin(run::new(self, arg))
    }
}
