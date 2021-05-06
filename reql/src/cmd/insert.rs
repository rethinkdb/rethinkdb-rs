use super::args::Args;
use crate::cmd::Durability;
use crate::Query;
use ql2::term::TermType;
use serde::{Serialize, Serializer};
use serde_json::Value;

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReturnChanges {
    Bool(bool),
    Always,
}

impl Serialize for ReturnChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Always => "always".serialize(serializer),
        }
    }
}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Insert).with_arg(self)
    }
}

impl Arg for Args<(Query, Options)> {
    fn into_query(self) -> Query {
        let Args((query, options)) = self;
        query.into_query().with_opts(options)
    }
}

impl Arg for Value {
    fn into_query(self) -> Query {
        Query::from(self).into_query()
    }
}

impl<T> Arg for Vec<T>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let arg = Value::Array(self.into_iter().map(Into::into).collect());
        Query::from(arg).into_query()
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let Args((val, options)) = self;
        Query::from(val.into()).into_query().with_opts(options)
    }
}
