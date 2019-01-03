use crate::cmd::run::ser::to_vec;
use bytes::Bytes;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) bytes: Bytes,
}

impl<T: Serialize> From<T> for Arg {
    fn from(t: T) -> Self {
        Arg {
            bytes: Bytes::from(to_vec(&t)),
        }
    }
}
