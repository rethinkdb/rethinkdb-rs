use crate::types::Binary;
use crate::{r, Query};
use ql2::term::TermType;
use serde_json::json;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        build(self)
    }
}

impl Arg for Binary {
    fn into_query(self) -> Query {
        build(r.expr(json!(self)))
    }
}

impl Arg for &[u8] {
    fn into_query(self) -> Query {
        Binary::new(self).into_query()
    }
}

impl Arg for &Vec<u8> {
    fn into_query(self) -> Query {
        Binary::new(self).into_query()
    }
}

impl Arg for Vec<u8> {
    fn into_query(self) -> Query {
        Binary::new(&self).into_query()
    }
}

fn build(arg: Query) -> Query {
    Query::new(TermType::Binary).with_arg(arg)
}
