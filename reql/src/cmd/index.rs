use super::asc::Asc;
use super::desc::Desc;
use crate::Query;
use serde_json::json;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Index(pub(crate) Query);

pub trait Arg {
    fn into_query(self) -> Query;
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        Query::from_json(json!({"index": self.into() }))
    }
}

impl Arg for Asc {
    fn into_query(self) -> Query {
        let Asc(query) = self;
        Query::from_json(json!({ "index": query }))
    }
}

impl Arg for Desc {
    fn into_query(self) -> Query {
        let Desc(query) = self;
        Query::from_json(json!({ "index": query }))
    }
}
