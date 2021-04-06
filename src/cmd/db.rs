use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;

pub(crate) fn new(name: String) -> Query {
    Query {
        typ: TermType::Db,
        args: vec![Datum::String(name).into()],
        ..Default::default()
    }
}
