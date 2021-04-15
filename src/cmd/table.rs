use crate::proto::Datum;
use crate::Query;
use ql2::term::TermType;

pub struct Options;

pub trait Arg {
    fn arg(self) -> (String, Option<Options>);
}

impl Arg for String {
    fn arg(self) -> (String, Option<Options>) {
        (self, None)
    }
}

impl Arg for &str {
    fn arg(self) -> (String, Option<Options>) {
        (self.to_owned(), None)
    }
}

impl Arg for &String {
    fn arg(self) -> (String, Option<Options>) {
        (self.to_owned(), None)
    }
}

pub(crate) fn new(parent: Option<Query>, (name, _opts): (String, Option<Options>)) -> Query {
    Query {
        typ: TermType::Table,
        change_feed: parent.as_ref().map(|x| x.change_feed).unwrap_or_default(),
        args: match parent {
            Some(parent) => vec![parent, Datum::String(name).into()],
            None => vec![Datum::String(name).into()],
        },
        ..Default::default()
    }
}
