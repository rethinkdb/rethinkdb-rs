use crate::Query;
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<Squash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changefeed_queue_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_initial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_states: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_offsets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_types: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Squash {
    Bool(bool),
    Float(f32),
}

pub trait Arg {
    fn arg(self) -> Option<Options>;
}

impl Arg for () {
    fn arg(self) -> Option<Options> {
        None
    }
}

impl Arg for Options {
    fn arg(self) -> Option<Options> {
        Some(self)
    }
}

pub(crate) fn new(parent: Query, opts: Option<Options>) -> Query {
    parent
        .append(TermType::Changes)
        .with_opts(opts)
        .mark_change_feed()
}
