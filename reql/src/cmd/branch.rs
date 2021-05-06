use super::args::Args;
use crate::Query;
use ql2::term::TermType;

pub trait Arg {
    fn into_query(self) -> Query;
}

impl Arg for Query {
    fn into_query(self) -> Query {
        Self::new(TermType::Branch).with_arg(self)
    }
}

impl Arg for Args<(Query, Query, Query)> {
    fn into_query(self) -> Query {
        let Args((test, true_action, false_action)) = self;
        test.into_query()
            .with_arg(true_action)
            .with_arg(false_action)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([(Query, Query); N], Query)> {
    fn into_query(self) -> Query {
        let Args((arr, false_action)) = self;
        let mut query = Query::new(TermType::Branch);
        // TODO remove the clone in Rust v1.53
        for (test, true_action) in arr.into_iter().cloned() {
            query = query.with_arg(test).with_arg(true_action);
        }
        query.with_arg(false_action)
    }
}
