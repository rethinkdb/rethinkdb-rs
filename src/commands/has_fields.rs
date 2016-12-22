#![allow(dead_code)]

use ql2::types;
use types::string::IntoString;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use serde_json::value::ToJson;

impl<O> Client<types::StreamSelection, O>
    where O: ToJson + Clone
{
    pub fn has_fields<T>(self, arg: T) -> Client<types::Stream, ()>
        where T: IntoString
        {
            super::make_cmd(TermType::HAS_FIELDS, Some(vec![arg.into_string()]), None, Some(self.cmd), self.errors)
        }
}

impl<O> Client<types::Object, O>
    where O: ToJson + Clone
{
    pub fn has_fields<T>(self, arg: T) -> Client<types::Bool, ()>
        where T: IntoString
        {
            super::make_cmd(TermType::HAS_FIELDS, Some(vec![arg.into_string()]), None, Some(self.cmd), self.errors)
        }
}

impl<O> Client<types::Array, O>
    where O: ToJson + Clone
{
    pub fn has_fields<T>(self, arg: T) -> Client<types::Array, ()>
        where T: IntoString
        {
            super::make_cmd(TermType::HAS_FIELDS, Some(vec![arg.into_string()]), None, Some(self.cmd), self.errors)
        }
}
