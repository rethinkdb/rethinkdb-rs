#![allow(dead_code)]

use ::Pool;
use ql2::types;
use ql2::proto::Term;
use super::Command;
use conn::Session;
use serde_json::value::ToJson;

#[derive(Debug, Clone)]
pub struct RunOpts;

impl Default for RunOpts {
    fn default() -> RunOpts {
        RunOpts
    }
}

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
