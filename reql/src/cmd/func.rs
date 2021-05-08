use crate::Query;
use ql2::term::TermType;

#[derive(Debug)]
pub struct Func(pub(crate) Query);

impl Func {
    pub fn new<T>(ids: Vec<u64>, body: T) -> Self
    where
        T: Into<Query>,
    {
        Func(
            Query::new(TermType::Func)
                .with_arg(Query::from_json(ids))
                .with_arg(body),
        )
    }

    pub(crate) fn row<T>(body: T) -> Self
    where
        T: Into<Query>,
    {
        Self::new(vec![0], body)
    }
}
