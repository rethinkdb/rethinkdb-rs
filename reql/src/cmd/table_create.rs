use super::args::Args;
use super::{Durability, StaticString};
use crate::Query;
use ql2::term::TermType;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct Options {
    pub primary_key: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn primary_key<T: StaticString>(mut self, primary_key: T) -> Self {
        self.primary_key = Some(primary_key.static_string());
        self
    }

    pub const fn durability(mut self, durability: Durability) -> Self {
        self.durability = Some(durability);
        self
    }

    pub const fn shards(mut self, shards: u8) -> Self {
        self.shards = Some(shards);
        self
    }

    pub fn replicas(mut self, replicas: Replicas) -> Self {
        self.replicas = Some(replicas);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Replicas {
    Int(u8),
    Map {
        replicas: HashMap<Cow<'static, str>, u8>,
        primary_replica_tag: Cow<'static, str>,
    },
}

impl Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct InnerOptions<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_key: Option<&'a Cow<'static, str>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            durability: Option<Durability>,
            #[serde(skip_serializing_if = "Option::is_none")]
            shards: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            replicas: Option<InnerReplicas<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_replica_tag: Option<&'a Cow<'static, str>>,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum InnerReplicas<'a> {
            Int(u8),
            Map(&'a HashMap<Cow<'static, str>, u8>),
        }

        let (replicas, primary_replica_tag) = match &self.replicas {
            Some(Replicas::Int(i)) => (Some(InnerReplicas::Int(*i)), None),
            Some(Replicas::Map {
                replicas,
                primary_replica_tag,
            }) => (
                Some(InnerReplicas::Map(replicas)),
                Some(primary_replica_tag),
            ),
            None => (None, None),
        };

        let opts = InnerOptions {
            replicas,
            primary_replica_tag,
            primary_key: self.primary_key.as_ref(),
            durability: self.durability,
            shards: self.shards,
        };

        opts.serialize(serializer)
    }
}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::TableCreate).with_arg(self)
    }
}

impl Arg for Args<(Query, Options)> {
    fn into_query(self) -> Query {
        let Args((query, options)) = self;
        query.into_query().with_opts(options)
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        Query::from_json(self.into()).into_query()
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        let Args((name, options)) = self;
        name.into_query().with_opts(options)
    }
}
