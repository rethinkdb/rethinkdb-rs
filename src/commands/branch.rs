#![allow(dead_code)]

use types;
use args::bool::IntoBool;
use args::term::IntoTerm;
use ql2::proto::{Term, Term_TermType as TermType};
use ::Client;
use serde_json::value::ToJson;

macro_rules! define {
    ($name:ident returns $typ:ident) => {
        impl Client<(), ()>
        {
            pub fn $name<B, T, F>(self, arg: Vec<(B, T)>, fallback: F) -> Client<types::$typ, ()>
                where B: IntoBool,
                      T: IntoTerm,
                      F: IntoTerm,
            {
                let mut args = Vec::new();
                for a in arg {
                    let test: Term = a.0.into_bool().into();
                    args.push(test);
                    args.push(a.1.into_term());
                }
                args.push(fallback.into_term());
                super::root_client(TermType::BRANCH, Some(args), None, self)
            }
        }

        impl<O> Client<types::Bool, O>
            where O: ToJson + Clone
        {
            pub fn $name<T, B, E, F>(self, arg: T, extra: Option<Vec<(B, E)>>, fallback: F) -> Client<types::$typ, ()>
                where T: IntoTerm,
                      B: IntoBool,
                      E: IntoTerm,
                      F: IntoTerm
            {
                let mut args = Vec::new();
                args.push(arg.into_term());
                if let Some(arg) = extra {
                    for a in arg {
                        let test: Term = a.0.into_bool().into();
                        args.push(test);
                        args.push(a.1.into_term());
                    }
                }
                args.push(fallback.into_term());
                let term: Term = args.into();
                let array = types::Array::from(term);
                super::root_client(TermType::BRANCH, Some(vec![array]), None, self)
            }
        }
    }
}

define! { string_branch returns String }
define! { number_branch returns Number }
