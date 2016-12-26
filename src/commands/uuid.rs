#![allow(dead_code)]

use types;
use ql2::proto::Term_TermType as TermType;
use ::Client;

impl Client<(), ()> {
    pub fn uuid(self) -> Client<types::String, ()> {
        super::root_client(TermType::UUID, NoArg!(), None, self)
    }
}
