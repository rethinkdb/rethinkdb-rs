#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use ::Client;
use serde_json::value::ToJson;
use types::string::IntoString;

macro_rules! define {
    ($name:ident returns $typ:ident) => {
        impl<O> Client<types::StreamSelection, O>
            where O: ToJson + Clone
            {
                pub fn $name<T>(self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoString
                    {
                        super::client(TermType::GET_FIELD, Some(vec![arg.into_string()]), None, self)
                    }
            }

        impl<O> Client<types::Object, O>
            where O: ToJson + Clone
            {
                pub fn $name<T>(self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoString
                    {
                        super::client(TermType::GET_FIELD, Some(vec![arg.into_string()]), None, self)
                    }
            }

        impl<O> Client<types::Array, O>
            where O: ToJson + Clone
            {
                pub fn $name<T>(self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoString
                    {
                        super::client(TermType::GET_FIELD, Some(vec![arg.into_string()]), None, self)
                    }
            }
    }
}

define! { string_field returns String }
define! { number_field returns Number }
