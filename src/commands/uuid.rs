#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use ::Client;

impl Client<(), ()> {
    pub fn uuid(self) -> Client<types::String, ()> {
        super::make_cmd(TermType::UUID, NoArg!(), None, Root!(), self.errors)
    }
}
