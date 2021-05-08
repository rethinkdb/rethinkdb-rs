use super::args::Args;
use super::ReadMode;
use crate::{cmd, Query};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
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
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::Table).with_arg(self).into_arg()
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

impl Arg for Args<(Query, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((query, options)) = self;
        query.arg().with_opts(options)
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
