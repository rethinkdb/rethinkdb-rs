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
    fn arg(self) -> (Value, Option<Options>);
}

impl Arg for Value {
    fn arg(self) -> (Value, Option<Options>) {
        (self, None)
    }
}

impl<T> Arg for Vec<T>
where
    T: Into<Value>,
{
    fn arg(self) -> (Value, Option<Options>) {
        (
            Value::Array(self.into_iter().map(Into::into).collect()),
            None,
        )
    }
}

impl<T> Arg for (T, Options)
where
    T: Into<Value>,
{
    fn arg(self) -> (Value, Option<Options>) {
        (self.0.into(), Some(self.1))
    }
}

pub(crate) fn new(parent: Query, (arg, opts): (Value, Option<Options>)) -> Query {
    parent
        .append(TermType::Insert)
        .with_arg(Datum::from(arg))
        .with_opts(opts)
}
