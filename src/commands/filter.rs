#![allow(dead_code)]

use ::Client;
use types;
use args::filter::IntoFilterArg;
use ql2::proto::{Term_TermType as TermType};
use serde_json::value::ToJson;

macro_rules! filter {
    (Selection for $arg:ident) => {
        impl<O> Client<types::Selection<types::$arg>, O>
            where O: ToJson + Clone
            {
                pub fn filter<T>(mut self, arg: T) -> Client<types::$arg, ()>
                    where T: IntoFilterArg<types::$arg, types::$arg>
                    {
                        super::client(TermType::FILTER, Some(arg.into_filter_arg(&mut self.idx)), None, self)
                    }
            }
    };

    ($arg:ident for $typ:ident) => {
        impl<O> Client<types::$arg, O>
            where O: ToJson + Clone
            {
                pub fn filter<T>(mut self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoFilterArg<types::$arg, types::$typ>
                    {
                        super::client(TermType::FILTER, Some(arg.into_filter_arg(&mut self.idx)), None, self)
                    }
            }
    };
}

filter!{ Stream for Stream }
filter!{ Selection for Stream }
filter!{ Table for Stream }
filter!{ Array for Array }
filter!{ Selection for Array }
