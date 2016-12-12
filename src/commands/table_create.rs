#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::{
    Command, TableCreateOpts,
    ReplicaArg, PrimaryKeyArg,
    Durability,
};
use serde_json::value::ToJson;

impl Command<(), ()>
{
    pub fn table_create<T>(&self, arg: T) -> Command<types::Object, TableCreateOpts<String, u64>>
        where T: Into<types::String>
    {
            let config = ::config().read();
            super::r.db(config.db()).table_create(arg)
    }
}

impl<O> Command<types::Db, O>
    where O: ToJson + Clone
{
    pub fn table_create<T>(&self, arg: T) -> Command<types::Object, TableCreateOpts<String, u64>>
        where T: Into<types::String>
    {
        super::make_cmd(TermType::TABLE_CREATE,
                  Some(vec![arg.into()]),
                  Some(TableCreateOpts::default()),
                  Some(self))
    }
}

impl<T, P, R> Command<T, TableCreateOpts<P, R>>
    where P: PrimaryKeyArg,
          R: ReplicaArg,
{
    pub fn primary_key<K>(self, arg: K) -> Command<T, TableCreateOpts<K, R>>
        where K: PrimaryKeyArg
    {
        let opts = self.1.expect("TableCreateOpts must be set during command construction");
        let opts = TableCreateOpts {
            primary_key: arg,
            durability: opts.durability,
            shards: opts.shards,
            replicas: opts.replicas,
        };
        Command(self.0, Some(opts))
    }

    pub fn durability(&mut self, arg: Durability) -> &mut Self {
        set_opt!(self, durability(arg));
        self
    }
    pub fn shards(&mut self, arg: u8) -> &mut Self {
        set_opt!(self, shards(arg));
        self
    }

    pub fn replicas<A>(self, arg: A) -> Command<T, TableCreateOpts<P, A>>
        where A: ReplicaArg
    {
        let opts = self.1.expect("TableCreateOpts must be set during command construction");
        let opts = TableCreateOpts {
            primary_key: opts.primary_key,
            durability: opts.durability,
            shards: opts.shards,
            replicas: arg,
        };
        Command(self.0, Some(opts))
    }
}
