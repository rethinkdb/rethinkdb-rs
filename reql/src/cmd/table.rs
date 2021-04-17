use super::ReadMode;
use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        build(self)
    }
}

impl Arg for String {
    fn into_query(self) -> Query {
        build(Datum::String(self))
    }
}

impl Arg for &String {
    fn into_query(self) -> Query {
        build(Datum::String(self.to_owned()))
    }
}

impl Arg for &str {
    fn into_query(self) -> Query {
        build(Datum::String(self.to_owned()))
    }
}

impl Arg for (Query, Options) {
    fn into_query(self) -> Query {
        build(self.0).with_opts(self.1)
    }
}

impl Arg for (String, Options) {
    fn into_query(self) -> Query {
        build(Datum::String(self.0)).with_opts(self.1)
    }
}

impl Arg for (&String, Options) {
    fn into_query(self) -> Query {
        build(Datum::String(self.0.to_owned())).with_opts(self.1)
    }
}

impl Arg for (&str, Options) {
    fn into_query(self) -> Query {
        build(Datum::String(self.0.to_owned())).with_opts(self.1)
    }
}

fn build<T>(arg: T) -> Query
where
    T: Into<Query>,
{
    Query::new(TermType::Table).with_arg(arg)
}
