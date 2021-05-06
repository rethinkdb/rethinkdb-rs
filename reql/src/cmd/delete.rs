use crate::Query;
use ql2::term::TermType;
use serde::Serialize;

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Options {}

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for () {
    fn into_query(self) -> Query {
        Query::new(TermType::Delete)
    }
}

impl Arg for Options {
    fn into_query(self) -> Query {
        ().into_query().with_opts(self)
    }
}
