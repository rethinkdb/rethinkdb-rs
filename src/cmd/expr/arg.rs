use {crate::ser::to_vec, bytes::Bytes, serde::Serialize};

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
