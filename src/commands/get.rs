use ql2::types;
use ql2::proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Table, O>
    where O: ToJson + Clone
{
    pub fn get<T>(&self, arg: T) -> Command<types::ObjectSelection, ()>
        where T: Into<types::PrimaryKey>
    {
        super::make_cmd(TermType::GET, Some(vec![arg.into()]), None, Some(self))
    }
}
