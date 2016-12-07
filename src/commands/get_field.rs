#![allow(dead_code)]

use ql2::types;
use ql2::types::Command as Cmd;
use ql2::proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

impl<O> Command<types::Object, O>
    where O: ToJson + Clone
{
    pub fn get_field<T, V>(&self, arg: T) -> Command<V, ()>
        where T: Into<types::String>,
              V: types::DataType
    {
        super::make_cmd(TermType::GET_FIELD,
                  Some(vec![arg.into()]),
                  None,
                  Some(self))
    }
}

impl<O> Command<types::Array, O>
    where O: ToJson + Clone
{
    pub fn get_field<T>(&self, arg: T) -> Command<types::Array, ()>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::GET_FIELD,
                  Some(vec![arg.into()]),
                  None,
                  Some(self))
    }
}
