use crate::cmd::run::{Db, Options};
use crate::{err, r};
use ql2::query::QueryType;
use ql2::term::TermType;
use serde::{ser, Serialize, Serializer};
use serde_json::value::{Number, Value};
use std::collections::HashMap;
use std::{fmt, str};

#[derive(Debug, Clone)]
pub(crate) enum Datum {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Datum>),
    Object(HashMap<String, Datum>),
}

impl Default for Datum {
    fn default() -> Self {
        Self::Null
    }
}

impl Serialize for Datum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Number(num) => num.serialize(serializer),
            Self::String(string) => string.serialize(serializer),
            Self::Array(arr) => (TermType::MakeArray as i32, arr).serialize(serializer),
            Self::Object(map) => map.serialize(serializer),
        }
    }
}

impl From<Value> for Datum {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(boolean) => Self::Bool(boolean),
            Value::Number(num) => Self::Number(num),
            Value::String(string) => Self::String(string),
            Value::Array(arr) => Self::Array(arr.into_iter().map(Into::into).collect()),
            Value::Object(map) => Self::Object(
                map.into_iter()
                    .map(|(key, value)| (key, value.into()))
                    .collect(),
            ),
        }
    }
}

/// The query that will be sent to RethinkDB
#[derive(Debug, Clone, Default)]
pub struct Query {
    typ: TermType,
    datum: Option<Datum>,
    args: Vec<Query>,
    opts: Option<Datum>,
    change_feed: bool,
}

fn to_datum<T>(opts: T) -> Datum
where
    T: Serialize,
{
    serde_json::to_value(&opts)
        // it's safe to unwrap here because we only use opts
        // types that are derived in this crate and we know
        // those don't return errors
        .unwrap()
        .into()
}

impl Query {
    pub(crate) fn new(typ: TermType) -> Self {
        Self {
            typ,
            ..Default::default()
        }
    }

    pub(crate) fn append(self, typ: TermType) -> Self {
        Self {
            typ,
            change_feed: self.change_feed,
            args: vec![self],
            ..Default::default()
        }
    }

    pub(crate) fn with_arg<T>(mut self, arg: T) -> Self
    where
        T: Into<Query>,
    {
        self.args.push(arg.into());
        self
    }

    pub(crate) fn with_opts<T>(mut self, opts: Option<T>) -> Self
    where
        T: Serialize,
    {
        self.opts = opts.map(to_datum);
        self
    }

    pub(crate) fn mark_change_feed(mut self) -> Self {
        self.change_feed = true;
        self
    }

    pub(crate) fn change_feed(&self) -> bool {
        self.change_feed
    }
}

impl From<Datum> for Query {
    fn from(datum: Datum) -> Self {
        Self {
            typ: TermType::Datum,
            datum: Some(datum),
            ..Default::default()
        }
    }
}

impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.typ {
            TermType::Datum => self.datum.serialize(serializer),
            _ => {
                let typ = self.typ as i32;
                match &self.opts {
                    Some(map) => (typ, &self.args, map).serialize(serializer),
                    None => (typ, &self.args).serialize(serializer),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Payload<'a>(
    pub(crate) QueryType,
    pub(crate) Option<Query>,
    pub(crate) Option<Options<'a>>,
);

impl Serialize for Payload<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let typ = self.0 as i32;
        match (&self.1, &self.2) {
            (Some(query), Some(opts)) => (typ, query, opts).serialize(serializer),
            (Some(query), None) => (typ, query).serialize(serializer),
            (None, None) => (typ,).serialize(serializer),
            (None, Some(_)) => Err(ser::Error::custom("unexpected options with no query")),
        }
    }
}

impl Payload<'_> {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, err::Error> {
        Ok(serde_json::to_vec(self)?)
    }
}

// for debugging purposes only
impl fmt::Display for Payload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the serialised string if we can
        if let Ok(payload) = self.to_bytes() {
            if let Ok(payload) = str::from_utf8(&payload) {
                return write!(f, "{}", payload);
            }
        }
        // otherwise just print the debug form
        write!(f, "{:?}", self)
    }
}

impl Serialize for Db<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        r.db(self.0).serialize(serializer)
    }
}
