use super::ReadMode;
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
        Query::new(TermType::Table).with_arg(self)
    }
}

impl Arg for String {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for &String {
    fn into_query(self) -> Query {
        self.to_owned().into_query()
    }
}

impl Arg for &str {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for (Query, Options) {
    fn into_query(self) -> Query {
        let (query, options) = self;
        query.into_query().with_opts(options)
    }
}

impl Arg for (String, Options) {
    fn into_query(self) -> Query {
        let (name, options) = self;
        name.into_query().with_opts(options)
    }
}

impl Arg for (&String, Options) {
    fn into_query(self) -> Query {
        let (name, options) = self;
        name.into_query().with_opts(options)
    }
}

impl Arg for (&str, Options) {
    fn into_query(self) -> Query {
        let (name, options) = self;
        name.into_query().with_opts(options)
    }
}
