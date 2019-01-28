use {crate::ser::to_vec, bytes::Bytes, serde::Serialize};

#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) arg: Bytes,
}

impl<T: Serialize> From<T> for Arg {
    fn from(t: T) -> Self {
        Arg {
            arg: Bytes::from(to_vec(&t)),
        }
    }
}
