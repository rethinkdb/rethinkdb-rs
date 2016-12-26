use ::{Client, Term};

pub trait IntoBool {
    fn into_bool(self) -> ::types::Bool;
}

impl<O> IntoBool for Client<::types::Bool, O> {
    fn into_bool(self) -> ::types::Bool {
        self.cmd.0
    }
}

impl<O> IntoBool for Client<::types::Null, O> {
    fn into_bool(self) -> ::types::Bool {
        Term::from_json(false).into()
    }
}
