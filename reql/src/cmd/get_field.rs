use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::GetField).with_arg(self)
    }
}

impl Arg for &str {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}

impl Arg for &String {
    fn into_query(self) -> Query {
        Query::from_json(self.as_str()).into_query()
    }
}

impl Arg for String {
    fn into_query(self) -> Query {
        Query::from_json(self).into_query()
    }
}
