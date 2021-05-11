use super::args::Args;
use crate::{cmd, Func, Query};
use ql2::term::TermType;
use serde::Serialize;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let arg = Query::from_json(self);
        Query::new(TermType::Funcall).with_arg(arg).into_arg()
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T> Arg for Args<(T, Query)>
where
    T: Serialize + Clone,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((args, expr)) = self;
        let arg = Query::from_json(args);
        expr.arg().with_arg(arg)
    }
}

impl<T> Arg for Args<(T, Func)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        let Args((args, Func(func))) = self;
        let args = Query::from_json(args);
        func.arg().with_arg(args)
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg()
    }
}
