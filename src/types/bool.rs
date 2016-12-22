use types::data;
use ::{Client, Term};

pub trait IntoBool {
    fn into_bool(self) -> data::Bool;
}

impl<O> IntoBool for Client<data::Bool, O> {
    fn into_bool(self) -> data::Bool {
        self.cmd.0
    }
}

impl<O> IntoBool for Client<data::Null, O> {
    fn into_bool(self) -> data::Bool {
        Term::from_json(false).into()
    }
}
