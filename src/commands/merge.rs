#![allow(dead_code)]

use ::Client;
use types;
use args::merge::IntoMergeArg;
use ql2::proto::{Term_TermType as TermType};
use serde_json::value::ToJson;

macro_rules! merge {
    (Selection for $arg:ident) => {
        impl<O> Client<types::Selection<types::$arg>, O>
            where O: ToJson + Clone
            {
                pub fn merge<T>(mut self, arg: T) -> Client<types::$arg, ()>
                    where T: IntoMergeArg<types::$arg, types::$arg>
                    {
                        super::client(TermType::MERGE, Some(arg.into_merge_arg(&mut self.idx)), None, self)
                    }
            }
    };

    ($arg:ident for $typ:ident) => {
        impl<O> Client<types::$arg, O>
            where O: ToJson + Clone
            {
                pub fn merge<T>(mut self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoMergeArg<types::$arg, types::$typ>
                    {
                        super::client(TermType::MERGE, Some(arg.into_merge_arg(&mut self.idx)), None, self)
                    }
            }
    };
}

merge!{ Array for Array }
merge!{ Selection for Array }
merge!{ Stream for Stream }
merge!{ Selection for Stream }
merge!{ Table for Stream }
merge!{ Selection for Object }
