use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) bytes: Bytes,
}

impl<T: Into<String>> From<T> for Arg {
    fn from(t: T) -> Self {
        Arg {
            bytes: Bytes::from(t.into()),
        }
    }
}
