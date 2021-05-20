use crate::Command;
use ql2::term::TermType;
use std::ops::Not;

impl Not for Command {
    type Output = Self;

    fn not(self) -> Self {
        Self::new(TermType::Not).with_arg(self)
    }
}
