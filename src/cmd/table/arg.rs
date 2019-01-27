use {super::opt::Opts, crate::ser::to_vec, bytes::Bytes};

#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) bytes: Bytes,
    pub(super) opts: Opts,
}

impl<'a> From<&'a str> for Arg {
    fn from(t: &'a str) -> Self {
        Arg {
            bytes: Bytes::from(to_vec(t)),
            opts: Default::default(),
        }
    }
}

impl From<String> for Arg {
    fn from(t: String) -> Self {
        Arg {
            bytes: Bytes::from(to_vec(&t)),
            opts: Default::default(),
        }
    }
}
