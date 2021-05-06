use crate::types::Binary;
use crate::{r, Query};
use ql2::term::TermType;
use serde_json::json;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Binary).with_arg(self)
    }
}

impl Arg for Binary {
    fn into_query(self) -> Query {
        r.expr(json!(self)).into_query()
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
