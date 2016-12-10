pub mod db;
pub mod table;
pub mod uuid;
pub mod get;
pub mod get_all;
pub mod changes;
pub mod map;
pub mod get_field;
pub mod rem;

use std::string::String as StdString;

use ::types::{self, Command as Cmd};
use ql2::proto::{Term, Term_TermType as TermType};
use serde_json::value::ToJson;

include!(concat!(env!("OUT_DIR"), "/opts.rs"));

/// Convenient type for use with maps
pub type Arg = Command<types::Object, ()>;

#[allow(non_upper_case_globals)]
pub const r: Command<(), ()> = Command((), None);

#[derive(Debug)]
pub struct Command<T, O>(T, Option<O>);

fn make_cmd<A, T, O, PT, PO>(typ: TermType,
                                 args: Option<Vec<A>>,
                                 opts: Option<O>,
                                 cmd: Option<&Command<PT, PO>>)
-> Command<T, O>
where A: types::DataType,
T: types::DataType,
O: ToJson + Clone,
PT: types::DataType,
PO: ToJson + Clone
{
    let (prev_cmd, prev_opts) = match cmd {
        Some(cmd) => (Some(cmd.0.clone().into()), cmd.1.clone()),
        None => (None, None),
    };
    let mut dt = Cmd::new(typ, prev_cmd);
    if let Some(args) = args {
        for arg in args {
            dt.with_args(arg.into());
        }
    }
    if let Some(opt) = prev_opts {
        let obj = types::Object::from(opt);
        dt.with_opts(obj);
    }
    Command(dt.into(), opts)
}

impl<T, O> From<Command<T, O>> for Term
where T: types::DataType,
O: ToJson + Clone
{
    fn from(t: Command<T, O>) -> Term {
        let term: Term = t.0.into();
        let mut cmd: Cmd = term.into();
        if let Some(opt) = t.1 {
            let obj = types::Object::from(opt);
            cmd.with_opts(obj);
        }
        cmd.into()
    }
}
