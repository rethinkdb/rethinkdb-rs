use {
    crate::{cmd::Param, ser::to_vec},
    bytes::Bytes,
    serde::Serialize,
};

#[derive(Debug, Clone)]
pub struct Arg {
    arg: Bytes,
    opts: Vec<u8>,
}

impl<T: Serialize> From<T> for Arg {
    fn from(t: T) -> Self {
        Arg {
            arg: Bytes::from(to_vec(&t)),
            opts: Vec::new(),
        }
    }
}

impl Param for Arg {
    fn arg(&self) -> &Bytes {
        &self.arg
    }

    fn opts(&self) -> &Vec<u8> {
        &self.opts
    }
}
