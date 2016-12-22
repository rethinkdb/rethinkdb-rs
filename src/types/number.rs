use types::data;
use ::Client;
use ql2::proto::Term;

pub trait IntoNumber {
    fn into_number(self) -> data::Number;
}

impl IntoNumber for f64 {
    fn into_number(self) -> data::Number {
        Term::from_json(self).into()
    }
}

impl<O> IntoNumber for Client<data::Number, O>
{
    fn into_number(self) -> data::Number {
        self.cmd.0
    }
}
