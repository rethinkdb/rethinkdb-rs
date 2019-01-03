use crate::cmd::run::ser::to_vec;
use bytes::Bytes;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Arg(pub(super) Bytes);

impl<T: Serialize> From<T> for Arg {
    fn from(t: T) -> Self {
        let bytes = to_vec(&t).unwrap();
        Arg(Bytes::from(bytes))
    }
}
