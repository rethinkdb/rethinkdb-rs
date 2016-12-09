#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Command;

impl Command<(), ()> {
    pub fn uuid(&self) -> Command<types::String, ()> {
        super::make_cmd(TermType::UUID, NoArg!(), None, Root!())
    }
}
