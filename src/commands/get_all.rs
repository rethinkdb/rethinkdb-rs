#![allow(dead_code)]

use ql2::types;
use types::secondary_key::IntoSecondaryKey;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use super::GetAllOpts;
use serde_json::value::ToJson;

impl<O> Client<types::Table, O>
    where O: ToJson + Clone
{
    pub fn get_all<T>(self, arg: T) -> Client<types::StreamSelection, GetAllOpts>
        where T: IntoSecondaryKey,
              GetAllOpts: ToJson + Clone
    {
        super::client(TermType::GET_ALL, Some(vec![arg.into_secondary_key()]), None, self)
    }
}

impl<T> Client<T, GetAllOpts> {
    pub fn index(mut self, arg: &str) -> Self {
        let opts = GetAllOpts { index: arg.to_string() };
        self.cmd.1 = Some(opts);
        self
    }
}
