#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Command;

impl Command<(), ()> {
    /// Create a database.
    pub fn db_create<T>(&self, arg: T) -> Command<types::Object, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::DB_CREATE, Some(vec![arg.into()]), None, Root!())
    }
}
