#![allow(dead_code)]

use crate::cmd::make_builder;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct Opts {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier_format: Option<IdentifierFormat>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

impl Opts {
    make_builder!();

    pub fn read_mode(&mut self, mode: ReadMode) -> &mut Self {
        self.read_mode = Some(mode);
        self
    }

    pub fn identifier_format(&mut self, fmt: IdentifierFormat) -> &mut Self {
        self.identifier_format = Some(fmt);
        self
    }
}
