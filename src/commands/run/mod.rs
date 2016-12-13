#![allow(dead_code)]

pub mod result;

use std::marker::PhantomData;
use std::collections::BTreeMap;
use std::result::Result as StdResult;

use errors::*;
use ::Pool;
use ql2::{types, Encode};
use ql2::proto::Term;
use super::{
    r,
    Client, Command, RunOpts,
    ReadMode, Format, Durability,
};
use conn::Session;
use serde_json::value::ToJson;
use serde::Deserialize;

const CHANNEL_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Query<S: Session, T: Deserialize> {
    term: Term,
    sess: S,
    resp: PhantomData<T>,
}

fn run<T, S, D, O>(client: Client<D, O>, pool: S) -> StdResult<Command<Query<S, T>, RunOpts>, Vec<Error>>
    where S: Session,
          T: Deserialize + Send,
          D: types::DataType,
          O: ToJson + Clone,
{
    let query = Query {
        term: client.cmd.into(),
        sess: pool,
        resp: PhantomData,
    };
    match client.errors {
        Some(errors) => Err(errors),
        None => Ok(Command(query, Some(RunOpts::default()))),
    }
}

pub trait Run {
    fn run<T>(self) -> StdResult<Command<Query<Pool, T>, RunOpts>, Vec<Error>>
        where T: Deserialize + Send;
}

pub trait RunWithConn {
    fn run<S, T>(self, arg: S) -> StdResult<Command<Query<S, T>, RunOpts>, Vec<Error>>
        where S: Session, T: Deserialize + Send;
}

macro_rules! define {
    ($typ:ty) => {
        impl<O> Run for Client<$typ, O>
            where O: ToJson + Clone
            {
                fn run<T>(self) -> StdResult<Command<Query<Pool, T>, RunOpts>, Vec<Error>>
                    where T: Deserialize + Send
                    {
                        run::<T, _, _, _>(self, Pool)
                    }
            }

        impl<O> RunWithConn for Client<$typ, O>
            where O: ToJson + Clone
            {
                fn run<S, T>(self, arg: S) -> StdResult<Command<Query<S, T>, RunOpts>, Vec<Error>>
                    where S: Session, T: Deserialize + Send
                    {
                        run::<T, _, _, _>(self, arg)
                    }
            }
    }
}

define!{ types::Array }
define!{ types::Binary }
define!{ types::Bool }
define!{ types::Function }
define!{ types::ObjectSelection }
define!{ types::ArraySelection }
define!{ types::StreamSelection }
define!{ types::Geometry }
define!{ types::GroupedData }
define!{ types::GroupedStream }
define!{ types::Table }
define!{ types::Object }
define!{ types::Stream }

impl Encode for RunOpts {
    fn encode(&self) -> String {
        let opts = self.clone();
        let mut o = BTreeMap::new();
        if let Some(read_mode) = opts.read_mode {
            o.insert("read_mode", Term::from_json(read_mode));
        }
        o.insert("time_format", Term::from_json(opts.time_format));
        o.insert("profile", Term::from_json(opts.profile));
        o.insert("durability", Term::from_json(opts.durability));
        o.insert("group_format", Term::from_json(opts.group_format));
        if let Some(db) = opts.db {
            let db: Term = db.into();
            o.insert("db", db);
        }
        o.insert("array_limit", Term::from_json(opts.array_limit));
        o.insert("binary_format", Term::from_json(opts.binary_format));
        o.insert("min_batch_rows", Term::from_json(opts.min_batch_rows));
        o.insert("max_batch_rows", Term::from_json(opts.max_batch_rows));
        o.insert("max_batch_bytes", Term::from_json(opts.max_batch_bytes));
        o.insert("max_batch_seconds", Term::from_json(opts.max_batch_seconds));
        o.insert("first_batch_scaledown_factor", Term::from_json(opts.first_batch_scaledown_factor));
        let opts: Term = o.into();
        opts.encode()
    }
}

impl<S, T> Command<Query<S, T>, RunOpts>
    where S: Session,
          T: Deserialize + Send,
{
    pub fn read_mode(&mut self, arg: ReadMode) -> &mut Self {
        let arg = Some(arg);
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
        let arg = Some(r.db(arg).cmd);
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