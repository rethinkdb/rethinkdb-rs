use ql2::types;
use ql2::proto::Term_TermType as TermType;
use types::primary_key::IntoPrimaryKey;
use ::Client;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Client<types::Table, O>
    where O: ToJson + Clone
{
    pub fn get<T>(self, arg: T) -> Client<types::ObjectSelection, ()>
        where T: IntoPrimaryKey
    {
        super::make_cmd(TermType::GET, Some(vec![arg.into_primary_key()]), None, Some(self.cmd), self.errors)
    }
}
