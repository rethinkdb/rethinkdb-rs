#![allow(dead_code)]

use ql2::types;
use types::IntoBool;
use ql2::proto::{Term, Term_TermType as TermType};
use ::Client;
use serde_json::value::ToJson;

macro_rules! define {
    ($name:ident returns $typ:ident) => {
        impl Client<(), ()>
        {
            pub fn $name<B, T, F>(self, arg: Vec<(B, T)>, fallback: F) -> Client<types::$typ, ()>
                where B: IntoBool,
                      T: Into<Term>,
                      F: Into<Term>,
            {
                let mut args = Vec::new();
                for a in arg {
                    let test: Term = a.0.into_bool().into();
                    args.push(test);
                    args.push(a.1.into());
                }
                args.push(fallback.into());
                super::make_cmd(TermType::BRANCH, Some(args), None, Root!(), self.errors)
            }
        }

        impl<O> Client<types::Bool, O>
            where O: ToJson + Clone
        {
            pub fn $name<T, B, E, F>(self, arg: T, extra: Option<Vec<(B, E)>>, fallback: F) -> Client<types::$typ, ()>
                where T: Into<Term>,
                      B: IntoBool,
                      E: Into<Term>,
                      F: Into<Term>
            {
                let mut args = Vec::new();
                args.push(arg.into());
                if let Some(arg) = extra {
                    for a in arg {
                        let test: Term = a.0.into_bool().into();
                        args.push(test);
                        args.push(a.1.into());
                    }
                }
                args.push(fallback.into());
                let term: Term = args.into();
                let array = types::Array::from(term);
                super::make_cmd(TermType::BRANCH, Some(vec![array]), None, Some(self.cmd), self.errors)
            }
        }
    }
}

define! { string_branch returns String }
define! { number_branch returns Number }
