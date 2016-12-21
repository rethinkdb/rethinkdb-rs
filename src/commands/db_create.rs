#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use types::IntoString;

impl Client<(), ()> {
    /// Create a database.
    pub fn db_create<T>(self, arg: T) -> Client<types::Object, ()>
        where T: IntoString
    {
        super::make_cmd(TermType::DB_CREATE, Some(vec![arg.into_string()]), None, Root!(), self.errors)
    }
}
