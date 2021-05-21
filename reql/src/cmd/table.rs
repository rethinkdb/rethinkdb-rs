use super::args::Args;
use super::ReadMode;
use crate::{cmd, Command};
use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<Options> {
        Self::new(TermType::Table).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        Command::from_json(self.into()).arg()
    }
}

impl Arg for Args<(Command, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((query, options)) = self;
        query.arg().with_opts(options)
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((name, options)) = self;
        name.arg().with_opts(options)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_table() {
        let query = r.table("foo");
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,["foo"]]"#;
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table() {
        let query = r.db("foo").table("bar");
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,[[14,["foo"]],"bar"]]"#;
        assert_eq!(serialised, expected);
    }
}
