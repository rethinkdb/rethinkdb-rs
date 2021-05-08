use crate::{cmd, Query};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for () {
    fn arg(self) -> cmd::Arg<()> {
        Query::new(TermType::ImplicitVar)
            .mark_implicit_var()
            .into_arg()
    }
}

impl Arg for Query {
    fn arg(self) -> cmd::Arg<()> {
        let mut arg = ().arg();
        arg.arg = arg.arg.bracket(self);
        arg
    }
}

impl Arg for String {
    fn arg(self) -> cmd::Arg<()> {
        let mut arg = ().arg();
        arg.arg = arg.arg.bracket(self);
        arg
    }
}

impl Arg for &String {
    fn arg(self) -> cmd::Arg<()> {
        let mut arg = ().arg();
        arg.arg = arg.arg.bracket(self.as_str());
        arg
    }
}

impl Arg for &str {
    fn arg(self) -> cmd::Arg<()> {
        let mut arg = ().arg();
        arg.arg = arg.arg.bracket(self);
        arg
    }
}
