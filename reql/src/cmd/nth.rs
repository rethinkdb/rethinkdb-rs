use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Query::new(TermType::Nth).with_arg(self)
    }
}

impl Arg for isize {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}
