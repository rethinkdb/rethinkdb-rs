use {
    crate::{cmd::Param, ser::to_vec},
    bytes::Bytes,
};

#[derive(Debug, Clone)]
pub struct Arg {
    arg: Bytes,
    opts: Vec<u8>,
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

impl Param for Arg {
    fn arg(&self) -> &Bytes {
        &self.arg
    }

    fn opts(&self) -> &Vec<u8> {
        &self.opts
    }
}
