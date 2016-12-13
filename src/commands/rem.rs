#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Client;
use serde_json::value::ToJson;

impl<O> Client<types::Number, O>
    where O: ToJson + Clone
{
    pub fn rem<T>(self, arg: T) -> Client<types::Number, ()>
        where T: Into<types::Number>
    {
        super::make_cmd(TermType::MOD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}
