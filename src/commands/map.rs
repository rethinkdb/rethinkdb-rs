#![allow(dead_code)]

use ql2::types;
use types::map::IntoMapArg;
use ql2::proto::{Term_TermType as TermType};
use ::Client;
use serde_json::value::ToJson;

macro_rules! map {
    ($in_typ:ident to $out_typ:ident for $typ:ident) => {
        impl<O> Client<types::$typ, O>
            where O: ToJson + Clone
            {
                pub fn map<T>(mut self, arg: T) -> Client<types::$out_typ, ()>
                    where T: IntoMapArg<types::$in_typ>
                    {
                        super::client(TermType::MAP, Some(arg.into_map_arg(&mut self.idx)), None, self)
                    }
            }
    }
}

map!{ Stream to Stream for Table }
map!{ Stream to Stream for Stream }
map!{ Stream to Stream for StreamSelection }
map!{ Array to Array for Array }
