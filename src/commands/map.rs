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
    }
}

map!{ Array for Array }
map!{ Stream for Stream }
map!{ Table for Stream }
map!{ StreamSelection for Stream }
