#![allow(dead_code)]

use ::Pool;
use ql2::types;
use ql2::proto::Term;
use super::{
    r,
    Command, RunOpts,
    ReadMode, Format, Durability,
};
use conn::Session;
use serde_json::value::ToJson;

#[derive(Debug)]
pub struct Query<S: Session> {
    term: Term,
    sess: S,
}

fn run<S, D, O>(cmd: Command<D, O>, pool: S) -> Command<Query<S>, RunOpts>
    where S: Session,
          D: types::DataType,
          O: ToJson + Clone,
{
    let query = Query {
        term: cmd.into(),
        sess: pool,
    };
    Command(query, Some(RunOpts::default()))
}

pub trait Run {
    fn run(self) -> Command<Query<Pool>, RunOpts>;
}

pub trait RunWithConn {
    fn run<T>(self, arg: T) -> Command<Query<T>, RunOpts>
        where T: Session;
}

impl<O> Run for Command<types::Table, O>
    where O: ToJson + Clone
{
    fn run(self) -> Command<Query<Pool>, RunOpts>
    {
        run(self, Pool)
    }
}

impl<O> RunWithConn for Command<types::Table, O>
    where O: ToJson + Clone
{
    fn run<T>(self, arg: T) -> Command<Query<T>, RunOpts>
        where T: Session
    {
        run(self, arg)
    }
}

impl<S> Command<Query<S>, RunOpts>
    where S: Session
{
    pub fn read_mode(&mut self, arg: ReadMode) -> &mut Self {
        set_opt!(self, read_mode(arg));
        self
    }

    pub fn time_format(&mut self, arg: Format) -> &mut Self {
        set_opt!(self, time_format(arg));
        self
    }

    pub fn profile(&mut self, arg: bool) -> &mut Self {
        set_opt!(self, profile(arg));
        self
    }

    pub fn durability(&mut self, arg: Durability) -> &mut Self {
        set_opt!(self, durability(arg));
        self
    }

    pub fn group_format(&mut self, arg: Format) -> &mut Self {
        set_opt!(self, group_format(arg));
        self
    }

    pub fn db(&mut self, arg: &str) -> &mut Self {
        let arg = Some(r.db(arg));
        set_opt!(self, db(arg));
        self
    }

    pub fn array_limit(&mut self, arg: u64) -> &mut Self {
        set_opt!(self, array_limit(arg));
        self
    }

    pub fn binary_format(&mut self, arg: Format) -> &mut Self {
        set_opt!(self, binary_format(arg));
        self
    }

    pub fn min_batch_rows(&mut self, arg: u32) -> &mut Self {
        set_opt!(self, min_batch_rows(arg));
        self
    }

    pub fn max_batch_rows(&mut self, arg: u64) -> &mut Self {
        set_opt!(self, max_batch_rows(arg));
        self
    }

    pub fn max_batch_bytes(&mut self, arg: u64) -> &mut Self {
        set_opt!(self, max_batch_bytes(arg));
        self
    }

    pub fn max_batch_seconds(&mut self, arg: f32) -> &mut Self {
        set_opt!(self, max_batch_seconds(arg));
        self
    }

    pub fn first_batch_scaledown_factor(&mut self, arg: u64) -> &mut Self {
        set_opt!(self, first_batch_scaledown_factor(arg));
        self
    }
}
