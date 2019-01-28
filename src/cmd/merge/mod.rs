mod arg;

use {super::Command, crate::cmd::expr::Expr, bytes::Bytes};

pub use arg::Arg;

#[derive(Debug, Clone)]
pub struct Merge {
    pub(super) bytes: Bytes,
}

impl Expr {
    pub fn merge<A>(&self, arg: A) -> Merge
    where
        A: Into<Arg>,
    {
        let Arg { arg } = arg.into();
        let cmd = Command::new(&self.bytes, 35, arg, Vec::new());
        Merge { bytes: cmd.into() }
    }
}
