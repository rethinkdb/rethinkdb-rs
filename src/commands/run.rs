#![allow(dead_code)]

use std::marker::PhantomData;
use std::iter::{IntoIterator, Iterator};

use ::{Pool, Result};
use ql2::types;
use ql2::proto::Term;
use super::{
    r,
    Command, RunOpts,
    ReadMode, Format, Durability,
};
use conn::{
    Session, ResponseValue,
};
use serde_json::value::ToJson;
use serde::Deserialize;

/// ReQL Response
///
/// Response returned by `run()`
pub struct Response<T: Deserialize>(Result<ResponseValue<T>>);

#[derive(Debug)]
pub struct Query<S: Session, T: Deserialize> {
    term: Term,
    sess: S,
    resp: PhantomData<T>,
}

fn run<T, S, D, O>(cmd: Command<D, O>, pool: S) -> Command<Query<S, T>, RunOpts>
    where S: Session,
          T: Deserialize,
          D: types::DataType,
          O: ToJson + Clone,
{
    let query = Query {
        term: cmd.into(),
        sess: pool,
        resp: PhantomData,
    };
    Command(query, Some(RunOpts::default()))
}

pub trait Run {
    fn run<T>(self) -> Command<Query<Pool, T>, RunOpts>
        where T: Deserialize;
}

pub trait RunWithConn {
    fn run<S, T>(self, arg: S) -> Command<Query<S, T>, RunOpts>
        where S: Session, T: Deserialize;
}

impl<O> Run for Command<types::Table, O>
    where O: ToJson + Clone
{
    fn run<T>(self) -> Command<Query<Pool, T>, RunOpts>
        where T: Deserialize
    {
        run::<T, _, _, _>(self, Pool)
    }
}

impl<O> RunWithConn for Command<types::Table, O>
    where O: ToJson + Clone
{
    fn run<S, T>(self, arg: S) -> Command<Query<S, T>, RunOpts>
        where S: Session, T: Deserialize
    {
        run::<T, _, _, _>(self, arg)
    }
}

impl<T> Iterator for Response<T>
    where T: Deserialize
{
    type Item = Result<ResponseValue<T>>;

    fn next(&mut self) -> Option<Result<ResponseValue<T>>> {
        unimplemented!();
    }
}

impl<S, T> IntoIterator for Command<Query<S, T>, RunOpts>
    where S: Session,
          T: Deserialize,
{
    type Item = Result<ResponseValue<T>>;
    type IntoIter = Response<T>;

    fn into_iter(self) -> Response<T> {
        unimplemented!();
    }
}

impl<S, T> Command<Query<S, T>, RunOpts>
    where S: Session,
          T: Deserialize,
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
