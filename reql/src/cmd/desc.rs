use crate::Query;
use ql2::term::TermType;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Desc(pub(crate) Query);

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Desc).with_arg(self)
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
