use {crate::ser::to_vec, bytes::Bytes};

#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) arg: Bytes,
    pub(super) opts: Vec<u8>,
}

impl<'a> From<&'a str> for Arg {
    fn from(t: &'a str) -> Self {
        Arg {
            arg: Bytes::from(to_vec(t)),
            opts: Vec::new(),
        }
    }
}

impl From<String> for Arg {
    fn from(t: String) -> Self {
        Arg {
            arg: Bytes::from(to_vec(&t)),
            opts: Vec::new(),
        }
    }
}
