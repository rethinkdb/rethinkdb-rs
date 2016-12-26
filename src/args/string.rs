use types;
use ::Client;
use ql2::proto::Term;

pub trait IntoString {
    fn into_string(self) -> types::String;
}

impl IntoString for String {
    fn into_string(self) -> types::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a String {
    fn into_string(self) -> types::String {
        Term::from_json(self).into()
    }
}

impl<'a> IntoString for &'a str {
    fn into_string(self) -> types::String {
        Term::from_json(self).into()
    }
}

impl<O> IntoString for Client<types::String, O>
{
    fn into_string(self) -> types::String {
        self.cmd.0
    }
}
