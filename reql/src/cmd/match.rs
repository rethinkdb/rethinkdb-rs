use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Match).with_arg(self)
    }
}