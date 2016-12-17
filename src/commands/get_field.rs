#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Client;
use serde_json::value::ToJson;

impl<O> Client<types::StreamSelection, O>
    where O: ToJson + Clone
{
    pub fn string_field<T>(self, arg: T) -> Client<types::String, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}

impl<O> Client<types::Object, O>
    where O: ToJson + Clone
{
    pub fn string_field<T>(self, arg: T) -> Client<types::String, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}

impl<O> Client<types::Array, O>
    where O: ToJson + Clone
{
    pub fn array_field<T>(self, arg: T) -> Client<types::Array, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}
