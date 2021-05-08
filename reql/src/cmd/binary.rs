use crate::types::Binary;
use crate::{cmd, r, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Binary).with_arg(self).into_arg()
    }
}

impl Arg for Binary {
    fn arg(self) -> cmd::Arg<()> {
        r.expr(self).arg()
    }
}

impl Arg for &[u8] {
    fn arg(self) -> cmd::Arg<()> {
        Binary::new(self).arg()
    }
}

impl Arg for &Vec<u8> {
    fn arg(self) -> cmd::Arg<()> {
        Binary::new(self).arg()
    }
}

impl Arg for Vec<u8> {
    fn arg(self) -> cmd::Arg<()> {
        Binary::new(&self).arg()
    }
}
