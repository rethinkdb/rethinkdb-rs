use super::args::Args;
use crate::{cmd, Command, Func};
use ql2::term::TermType;
use serde::Serialize;

pub trait Arg {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let cmd = Command::new(TermType::Funcall).with_arg(self);
        match parent {
            Some(parent) => cmd.with_arg(parent).into_arg(),
            None => cmd.into_arg(),
        }
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        Command::from_json(self).arg(parent)
    }
}

impl Arg for Args<(Command, Command)> {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((arg, expr)) = self;
        expr.arg(parent).with_arg(arg)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([Command; N], Command)> {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((args, expr)) = self;
        let mut cmd = expr.arg(parent);
        for arg in args.into_iter().cloned() {
            cmd = cmd.with_arg(arg);
        }
        cmd
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Command)>
where
    T: Serialize + Clone,
{
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((args, expr)) = self;
        let mut cmd = expr.arg(parent);
        for arg in args.into_iter().cloned() {
            let arg = Command::from_json(arg);
            cmd = cmd.with_arg(arg);
        }
        cmd
    }
}

impl Arg for Args<(Command, Func)> {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((arg, Func(func))) = self;
        func.arg(parent).with_arg(arg)
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> Arg for Args<([Command; N], Func)> {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((args, Func(func))) = self;
        let mut cmd = func.arg(parent);
        for arg in args.into_iter().cloned() {
            cmd = cmd.with_arg(arg);
        }
        cmd
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<T, const N: usize> Arg for Args<([T; N], Func)>
where
    T: Serialize + Clone,
{
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Args((args, Func(func))) = self;
        let mut cmd = func.arg(parent);
        for arg in args.into_iter().cloned() {
            let arg = Command::from_json(arg);
            cmd = cmd.with_arg(arg);
        }
        cmd
    }
}

impl Arg for Func {
    fn arg(self, parent: Option<Command>) -> cmd::Arg<()> {
        let Func(func) = self;
        func.arg(parent)
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as reql, cmd, func, r};

    #[test]
    fn r_do() {
        let counter = crate::current_counter();
        let query = r.do_(r.args(([10, 20], func!(|x, y| x + y))));
        let serialised = cmd::serialise(&query);
        let expected = format!(
            r#"[64,[[69,[[2,[2,3]],[24,[[10,[{}]],[10,[{}]]]]]],10,20]]"#,
            counter,
            counter + 1
        );
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table_get_do() {
        let counter = crate::current_counter();
        let query = r
            .db("mydb")
            .table("table1")
            .get("johndoe@example.com")
            .do_(func!(|doc| r
                .db("mydb")
                .table("table2")
                .get(doc.get_field("id"))));
        let serialised = cmd::serialise(&query);
        let expected = format!(
            r#"[64,[[69,[[2,[1]],[16,[[15,[[14,["mydb"]],"table2"]],[31,[[10,[{}]],"id"]]]]]],[16,[[15,[[14,["mydb"]],"table1"]],"johndoe@example.com"]]]]"#,
            counter
        );
        assert_eq!(serialised, expected);
    }
}
