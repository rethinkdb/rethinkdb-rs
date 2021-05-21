use crate::{cmd, Command};
use ql2::term::TermType;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Db).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<()> {
        Command::from_json(self.into()).arg()
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db() {
        let query = r.db("foo");
        let serialised = cmd::serialise(&query);
        let expected = r#"[14,["foo"]]"#;
        assert_eq!(serialised, expected);
    }
}
