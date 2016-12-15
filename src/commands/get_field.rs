#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Client;
use serde_json::value::ToJson;

impl<O> Client<types::StreamSelection, O>
    where O: ToJson + Clone
{
    pub fn get_field<T, V>(self, arg: T) -> Client<V, ()>
        where T: Into<types::String>,
              V: types::DataType,
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}

impl<O> Client<types::Object, O>
    where O: ToJson + Clone
{
    pub fn get_field<T>(self, arg: T) -> Client<types::Object, ()>
        where T: Into<types::String>,
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}

impl<O> Client<types::Array, O>
    where O: ToJson + Clone
{
    pub fn get_field<T>(self, arg: T) -> Client<types::Array, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::GET_FIELD, Some(vec![arg.into()]), None, Some(self.cmd), self.errors)
    }
}
