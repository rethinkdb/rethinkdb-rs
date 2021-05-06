use super::args::Args;
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
        Self::new(TermType::Table).with_arg(self)
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

impl Arg for Args<(Query, Options)> {
    fn into_query(self) -> Query {
        let Args((query, options)) = self;
        query.into_query().with_opts(options)
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
