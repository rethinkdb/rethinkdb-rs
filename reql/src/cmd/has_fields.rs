use crate::Query;
use ql2::term::TermType;
use serde_json::Value;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::HasFields).with_arg(self)
    }
}

impl<T> Arg for T
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}
