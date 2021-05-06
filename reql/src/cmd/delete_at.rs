use super::args::Args;
use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::DeleteAt).with_arg(self)
    }
}

impl Arg for i64 {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for Args<[i64; 2]> {
    fn into_query(self) -> Query {
        let Args([offset, end_offset]) = self;
        Query::from_json(offset)
            .into_query()
            .with_arg(Query::from_json(end_offset))
    }
}
