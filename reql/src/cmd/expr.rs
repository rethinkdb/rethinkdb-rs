use crate::Query;
use serde_json::Value;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        self
    }
}

impl<T> Arg for T
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        Query::from_json(self)
    }
}
