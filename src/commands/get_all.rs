#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::{Command, GetAllOpts};
use serde_json::value::ToJson;

impl<O> Command<types::Table, O>
    where O: ToJson + Clone
{
    pub fn get_all<T>(&self, arg: T) -> Command<types::StreamSelection, GetAllOpts>
        where T: Into<types::SecondaryKey>,
              GetAllOpts: ToJson + Clone
    {
        super::make_cmd(TermType::GET_ALL,
                  Some(vec![arg.into()]),
                  None,
                  Some(self))
    }
}

impl<T> Command<T, GetAllOpts> {
    pub fn index(mut self, arg: &str) -> Self {
        let opts = GetAllOpts { index: arg.to_string() };
        self.1 = Some(opts);
        self
    }
}
