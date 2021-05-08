use super::args::Args;
use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::DeleteAt).with_arg(self).into_arg()
    }
}

impl Arg for i64 {
    fn arg(self) -> cmd::Arg<()> {
        Query::from_json(self).arg()
    }
}

impl Arg for Args<[i64; 2]> {
    fn arg(self) -> cmd::Arg<()> {
        let Args([offset, end_offset]) = self;
        Query::from_json(offset)
            .arg()
            .with_arg(Query::from_json(end_offset))
    }
}
