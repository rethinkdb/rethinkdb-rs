use crate::proto::Datum;
use crate::Query;
use serde_json::Value;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl<T> Arg for T
where
    T: Into<Value>,
{
    fn into_query(self) -> Query {
        Datum::from(self.into()).into()
    }
}
