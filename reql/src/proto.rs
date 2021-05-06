use crate::cmd::run::{Db, Options};
use crate::{err, r};
use ql2::query::QueryType;
use ql2::term::TermType;
use serde::{Serialize, Serializer};
use serde_json::value::{Number, Value};
use std::collections::{HashMap, VecDeque};
use std::{fmt, str};

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> From<[Query; N]> for Query {
    fn from(arr: [Query; N]) -> Self {
        let mut query = Self::new(TermType::MakeArray);
        query.args = arr
            .into_iter()
            // TODO remove this clone on Rust v1.53 once
            // https://twitter.com/m_ou_se/status/1385966446254166020
            // is available on stable
            .cloned()
            .collect();
        query
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

#[derive(Debug)]
pub struct Func(pub Query);

impl From<Func> for Query {
    fn from(Func(func): Func) -> Self {
        func
    }
}

/// The query that will be sent to RethinkDB
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Query {
    typ: TermType,
    datum: Option<Datum>,
    args: VecDeque<Query>,
    opts: Option<Datum>,
    change_feed: bool,
}

impl Query {
    #[doc(hidden)]
    pub fn new(typ: TermType) -> Self {
        Self {
            typ,
            datum: None,
            args: VecDeque::new(),
            opts: None,
            change_feed: false,
        }
    }

    #[doc(hidden)]
    pub fn var(id: u64) -> Self {
        let index = Self::from_json(id);
        Self::new(TermType::Var).with_arg(index)
    }

    pub(crate) fn with_parent(mut self, parent: Query) -> Self {
        self.change_feed = self.change_feed || parent.change_feed;
        self.args.push_front(parent);
        self
    }

    #[doc(hidden)]
    pub fn with_arg<T>(mut self, arg: T) -> Self
    where
        T: Into<Query>,
    {
        self.args.push_back(arg.into());
        self
    }

    pub(crate) fn with_opts<T>(mut self, opts: T) -> Self
    where
        T: Serialize,
    {
        let opts = serde_json::to_value(&opts)
            // it's safe to unwrap here because we only use opts
            // types that are derived in this crate and we know
            // those don't return errors
            .unwrap()
            .into();
        self.opts = Some(opts);
        self
    }

    #[doc(hidden)]
    pub fn from_json<T>(arg: T) -> Self
    where
        T: Into<Value>,
    {
        arg.into().into()
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
        let mut query = Self::new(TermType::Datum);
        query.datum = Some(datum);
        query
    }
}

impl From<Value> for Query {
    fn from(value: Value) -> Self {
        Datum::from(value).into()
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
pub(crate) struct Payload(
    pub(crate) QueryType,
    pub(crate) Option<Query>,
    pub(crate) Options,
);

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Payload(typ, qry, opts) = self;
        let typ = *typ as i32;
        match qry {
            Some(query) => (typ, query, opts).serialize(serializer),
            None => (typ,).serialize(serializer),
        }
    }
}

impl Payload {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, err::Error> {
        Ok(serde_json::to_vec(self)?)
    }
}

// for debugging purposes only
impl fmt::Display for Payload {
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

impl Serialize for Db {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Self(name) = self;
        r.db(name.as_ref()).serialize(serializer)
    }
}
