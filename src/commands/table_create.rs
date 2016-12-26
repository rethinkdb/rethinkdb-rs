#![allow(dead_code)]

use types;
use args::string::IntoString;
use ql2::proto::Term_TermType as TermType;
use ::{Client, Command};
use super::{
    Durability,
    TableCreateOpts,
    ReplicaArg, PrimaryKeyArg,
};
use serde_json::value::ToJson;

impl Client<(), ()>
{
    pub fn table_create<T>(self, arg: T) -> Client<types::Object, TableCreateOpts<String, u64>>
        where T: IntoString
    {
            let config = ::config().read();
            super::r.db(config.db()).table_create(arg)
    }
}

impl<O> Client<types::Db, O>
    where O: ToJson + Clone
{
    pub fn table_create<T>(self, arg: T) -> Client<types::Object, TableCreateOpts<String, u64>>
        where T: IntoString
    {
        super::client(TermType::TABLE_CREATE, Some(vec![arg.into_string()]), Some(TableCreateOpts::default()), self)
    }
}

impl<T, P, R> Client<T, TableCreateOpts<P, R>>
    where P: PrimaryKeyArg,
          R: ReplicaArg,
          T: types::DataType,
          TableCreateOpts<P, R>: ToJson,
{
    pub fn primary_key<K>(self, arg: K) -> Client<T, TableCreateOpts<K, R>>
        where K: PrimaryKeyArg
    {
        let opts = self.cmd.opts();
        let opts = TableCreateOpts {
            primary_key: arg,
            durability: opts.durability,
            shards: opts.shards,
            replicas: opts.replicas,
        };
        Client {
            cmd: Command(self.cmd.0, Some(opts)),
            idx: self.idx,
            errors: self.errors,
        }
    }

    pub fn durability(mut self, arg: Durability) -> Self {
        let mut opts = self.cmd.opts();
        opts.durability = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn shards(mut self, arg: u8) -> Self {
        let mut opts = self.cmd.opts();
        opts.shards = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn replicas<A>(self, arg: A) -> Client<T, TableCreateOpts<P, A>>
        where A: ReplicaArg
    {
        let opts = self.cmd.opts();
        let opts = TableCreateOpts {
            primary_key: opts.primary_key,
            durability: opts.durability,
            shards: opts.shards,
            replicas: arg,
        };
        Client {
            cmd: Command(self.cmd.0, Some(opts)),
            idx: self.idx,
            errors: self.errors,
        }
    }
}
