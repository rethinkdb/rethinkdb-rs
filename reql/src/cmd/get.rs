use crate::{cmd, Command};
use ql2::term::TermType;
use serde::Serialize;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Command::new(TermType::Get).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self).arg()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db_table_get() {
        let query = r
            .db("foo")
            .table("bar")
            .get("84fc23ac-9e85-43af-b6f7-f86be17237e1");
        let serialised = cmd::serialise(&query);
        let expected = r#"[16,[[15,[[14,["foo"]],"bar"]],"84fc23ac-9e85-43af-b6f7-f86be17237e1"]]"#;
        assert_eq!(serialised, expected);
    }
}
