use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;

pub(crate) fn new(name: String) -> Query {
    Query::new(TermType::Db).with_arg(Datum::String(name))
}
