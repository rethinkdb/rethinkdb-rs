#![allow(dead_code)]

use types;
use args::number::IntoNumber;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use serde_json::value::ToJson;

impl<O> Client<types::Number, O>
    where O: ToJson + Clone
{
    pub fn rem<T>(self, arg: T) -> Client<types::Number, ()>
        where T: IntoNumber
    {
        super::root_client(TermType::MOD, Some(vec![arg.into_number()]), None, self)
    }
}
