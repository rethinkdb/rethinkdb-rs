use crate::cmd::Durability;
use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;
use serde::{Serialize, Serializer};
use serde_json::Value;

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
        build(self)
    }
}

impl Arg for (Query, Options) {
    fn into_query(self) -> Query {
        build(self.0).with_opts(self.1)
    }
}

impl Arg for Value {
    fn into_query(self) -> Query {
        build(Datum::from(self))
    }
}

impl<T> Arg for Vec<T>
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        let arg = Value::Array(self.into_iter().map(Into::into).collect());
        build(Datum::from(arg))
    }
}

impl<T> Arg for (T, Options)
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        build(Datum::from(self.0.into())).with_opts(self.1)
    }
}

fn build<T>(arg: T) -> Query
where
    T: Into<Query>,
{
    Query::new(TermType::Insert).with_arg(arg)
}
