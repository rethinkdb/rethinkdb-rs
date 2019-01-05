use super::{Connection, Opts};

#[derive(Debug, Clone, Copy)]
pub struct Arg<'a> {
    pub(super) conn: &'a Connection,
    pub(super) opts: Option<Opts<'a>>,
}

impl<'a> From<&'a Connection> for Arg<'a> {
    fn from(conn: &'a Connection) -> Self {
        Self { conn, opts: None }
    }
}

impl<'a> From<(&'a Connection, Opts<'a>)> for Arg<'a> {
    fn from((conn, opts): (&'a Connection, Opts<'a>)) -> Self {
        Self {
            conn,
            opts: Some(opts),
        }
    }
}
