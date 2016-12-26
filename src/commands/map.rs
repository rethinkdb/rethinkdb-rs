#![allow(dead_code)]

use ::Client;
use types;
use args::map::{IntoRootMapArg, IntoMapArg};
use ql2::proto::{Term_TermType as TermType};
use serde_json::value::ToJson;

impl Client<(), ()>
{
    pub fn map<T, I, C>(mut self, arg: T) -> Client<C, ()>
        where T: IntoRootMapArg<I, C>,
              I: types::DataType,
              C: types::DataType,
    {
        super::root_client(TermType::MAP, Some(arg.into_map_arg(&mut self.idx)), None, self)
    }
}

macro_rules! map {
    (Selection for $arg:ident) => {
        impl<O> Client<types::Selection<types::$arg>, O>
            where O: ToJson + Clone
            {
                pub fn map<T>(mut self, arg: T) -> Client<types::$arg, ()>
                    where T: IntoMapArg<types::$arg, types::$arg>
                    {
                        super::client(TermType::MAP, Some(arg.into_map_arg(&mut self.idx)), None, self)
                    }
            }
    };

    ($arg:ident for $typ:ident) => {
        impl<O> Client<types::$arg, O>
            where O: ToJson + Clone
            {
                pub fn map<T>(mut self, arg: T) -> Client<types::$typ, ()>
                    where T: IntoMapArg<types::$arg, types::$typ>
                    {
                        super::client(TermType::MAP, Some(arg.into_map_arg(&mut self.idx)), None, self)
                    }
            }
    };
}

map!{ Array for Array }
map!{ Selection for Array }
map!{ Stream for Stream }
map!{ Selection for Stream }
map!{ Table for Stream }
