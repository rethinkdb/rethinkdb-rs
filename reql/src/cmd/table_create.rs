use super::args::Args;
use super::Durability;
use crate::{cmd, Query};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, CommandOptions, Default, PartialEq)]
#[non_exhaustive]
pub struct Options {
    pub primary_key: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
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
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::TableCreate).with_arg(self).into_arg()
    }
}

impl Arg for Args<(Query, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((query, options)) = self;
        query.arg().with_opts(options)
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        Query::from_json(self.into()).arg()
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, options)) = self;
        name.arg().with_opts(options)
    }
}
