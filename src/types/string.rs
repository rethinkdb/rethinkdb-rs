use types::data;
use ::Client;
use ql2::proto::Term;

pub trait IntoString {
    fn into_string(self) -> data::String;
}

impl IntoString for String {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a String {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a str {
    fn into_string(self) -> data::String {
        Term::from_json(self).into()
    }
}

impl<O> IntoString for Client<data::String, O>
{
    fn into_string(self) -> data::String {
        self.cmd.0
    }
}
