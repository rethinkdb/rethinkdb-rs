use crate::proto::Datum;
use crate::Query;
use serde_json::Value;

pub(crate) fn new(value: Value) -> Query {
    Datum::from(value).into()
}
