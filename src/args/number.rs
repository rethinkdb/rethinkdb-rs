use ::Client;
use types::Number;
use ql2::proto::Term;

pub trait IntoNumber {
    fn into_number(self) -> Number;
}

impl IntoNumber for f64 {
    fn into_number(self) -> Number {
        Term::from_json(self).into()
    }
}

impl<O> IntoNumber for Client<Number, O>
{
    fn into_number(self) -> Number {
        self.cmd.0
    }
}
