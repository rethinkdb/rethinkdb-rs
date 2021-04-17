use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        build(self)
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn into_query(self) -> Query {
        build(Datum::String(self.into()))
    }
}

fn build<T>(arg: T) -> Query
where
    T: Into<Query>,
{
    Query::new(TermType::DbCreate).with_arg(arg)
}
