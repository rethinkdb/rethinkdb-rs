use crate::cmd::make_builder;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct Opts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) db: Option<Db<'a>>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Format {
    Native,
    Raw,
}

impl<'a> Opts<'a> {
    make_builder!();

    pub fn db(&mut self, name: &'a str) -> &mut Self {
        self.db = Some(Db(name));
        self
    }

    pub fn profile(&mut self, profile: bool) -> &mut Self {
        self.profile = Some(profile);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Db<'a>(&'a str);

impl<'a> Serialize for Db<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (14, vec![self.0]).serialize(serializer)
    }
}
