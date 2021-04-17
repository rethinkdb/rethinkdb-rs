use crate::Query;

pub trait Arg {
    fn into_query(self) -> Query;
}
