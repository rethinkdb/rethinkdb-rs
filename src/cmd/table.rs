use crate::cmd::ReadMode;
use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

pub trait Arg {
    fn arg(self) -> (String, Option<Options>);
}

impl Arg for String {
    fn arg(self) -> (String, Option<Options>) {
        (self, None)
    }
}

impl Arg for &str {
    fn arg(self) -> (String, Option<Options>) {
        (self.to_owned(), None)
    }
}

impl Arg for &String {
    fn arg(self) -> (String, Option<Options>) {
        (self.to_owned(), None)
    }
}

pub(crate) fn new(parent: Option<Query>, (name, opts): (String, Option<Options>)) -> Query {
    match parent {
        Some(parent) => parent.append(TermType::Table),
        None => Query::new(TermType::Table),
    }
    .with_arg(Datum::String(name))
    .with_opts(opts)
}
