pub mod connection;
pub mod db_create;
pub mod db;
pub mod table_create;
pub mod table;
pub mod uuid;
pub mod get;
pub mod get_all;
pub mod changes;
pub mod map;
pub mod get_field;
pub mod has_fields;
pub mod branch;
pub mod rem;
pub mod run;

use std::string::String as StdString;
use std::sync::Arc;

use errors::Error;
use ::{Client, Command};
use types;
use ql2::proto::{Term,
    Term_TermType as TermType,
    Term_AssocPair as TermPair,
};
use serde_json::value::ToJson;
use protobuf::repeated::RepeatedField;

include!(concat!(env!("OUT_DIR"), "/opts.rs"));

/// Convenient type for use with maps
pub type Arg = Client<types::Object, ()>;

#[allow(non_upper_case_globals)]
pub const r: Client<(), ()> = Client {
    cmd: Command((), None),
    idx: 0,
    errors: None,
};

fn make_cmd<A, T, O, PT, PO>(typ: TermType,
                                 args: Option<Vec<A>>,
                                 opts: Option<O>,
                                 cmd: Option<Command<PT, PO>>,
                                 errors: Option<Arc<Vec<Error>>>,
                                 idx: u32)
-> Client<T, O>
where A: Into<Term>,
T: types::DataType,
O: ToJson + Clone,
PT: types::DataType,
PO: ToJson + Clone
{
    let prev_cmd = match cmd {
        Some(cmd) => {
            let cmd: Term = cmd.into();
            Some(cmd)
        },
        None => None,
    };
    let mut dt = Command::new(typ, prev_cmd);
    if let Some(args) = args {
        for arg in args {
            dt.with_args(arg.into());
        }
    }
    Client {
        cmd: Command(dt.into(), opts),
        idx: idx,
        errors: errors,
    }
}

fn client<A, T, O, PT, PO>(typ: TermType,
                                 args: Option<Vec<A>>,
                                 opts: Option<O>,
                                 client: Client<PT, PO>)
-> Client<T, O>
where A: Into<Term>,
T: types::DataType,
O: ToJson + Clone,
PT: types::DataType,
PO: ToJson + Clone
{
    make_cmd(typ, args, opts, Some(client.cmd), client.errors, client.idx)
}

fn root_client<A, T, O, PT, PO>(typ: TermType,
                                 args: Option<Vec<A>>,
                                 opts: Option<O>,
                                 client: Client<PT, PO>)
-> Client<T, O>
where A: Into<Term>,
T: types::DataType,
O: ToJson + Clone,
//PT: types::DataType,
PO: ToJson + Clone
{
    make_cmd(typ, args, opts, None as Option<Command<types::Null, ()>>, client.errors, client.idx)
}

impl<T, O> From<Command<T, O>> for Term
    where T: types::DataType,
          O: ToJson + Clone
{
    fn from(t: Command<T, O>) -> Term {
        let term: Term = t.0.into();
        let mut cmd = Command(term, None);
        if let Some(opt) = t.1 {
            let obj = types::Object::from(opt);
            cmd.with_opts(obj);
        }
        cmd.into()
    }
}

impl<T, O> From<Client<T, O>> for Term
    where T: types::DataType,
          O: ToJson + Clone
{
    fn from(t: Client<T, O>) -> Term {
        t.cmd.into()
    }
}

impl<T> From<T> for Client<T, ()> {
    fn from(t: T) -> Client<T, ()>
    {
        Client {
            cmd: Command(t, None as Option<()>),
            idx: 0,
            errors: None,
        }
    }
}


impl Command<Term, ()>
{
    pub fn new(cmd_type: TermType, prev_cmd: Option<Term>) -> Command<Term, ()> {
        let mut term = Term::new();
        term.set_field_type(cmd_type);
        if let Some(cmd) = prev_cmd {
            let args = RepeatedField::from_vec(vec![cmd]);
            term.set_args(args);
        }
        Command(term, None)
    }

    pub fn with_args(&mut self, args: Term) -> &mut Self {
        self.0.mut_args().push(args);
        self
    }

    pub fn with_opts(&mut self, opts: types::Object) -> &mut Self {
        let mut opts: Term = opts.into();
        if opts.has_datum() {
            let mut datum = opts.take_datum();
            let pairs = datum.take_r_object().into_vec();
            for mut pair in pairs {
                if pair.has_key() {
                    let mut term_pair = TermPair::new();
                    term_pair.set_key(pair.take_key());
                    let mut val = Term::new();
                    val.set_field_type(TermType::DATUM);
                    val.set_datum(pair.take_val());
                    term_pair.set_val(val);
                    self.0.mut_optargs().push(term_pair);
                }
            }
        }
        self
    }

    pub fn into<O>(self) -> O
        where O: From<Term>
    {
        From::from(self.0)
    }
}

impl<T, O> Command<T, O>
    where O: Clone
{
    fn opts(&self) -> O {
        let msg = "Command options is not set. This is a bug in the driver.";
        self.1.clone().expect(msg)
    }
}

#[derive(Debug, Clone)]
pub struct RunOpts {
    read_mode: Option<ReadMode>,
    time_format: Format,
    profile: bool,
    durability: Durability,
    group_format: Format,
    db: Option<Command<types::Db, ()>>,
    array_limit: u64,
    binary_format: Format,
    min_batch_rows: u32,
    max_batch_rows: u64,
    max_batch_bytes: u64,
    max_batch_seconds: f32,
    first_batch_scaledown_factor: u64,
}

impl Default for RunOpts {
    fn default() -> RunOpts {
        RunOpts {
            read_mode: Some(ReadMode::Single),
            time_format: Format::Native,
            profile: false,
            durability: Durability::Hard,
            group_format: Format::Native,
            db: None,
            array_limit: 100_000,
            binary_format: Format::Native,
            min_batch_rows: 8,
            // 2^53 is the biggest integer that RethinkDB supports
            max_batch_rows: 2u64.pow(53),
            max_batch_bytes: 1_000_000,
            max_batch_seconds: 0.5,
            first_batch_scaledown_factor: 4,
        }
    }
}
