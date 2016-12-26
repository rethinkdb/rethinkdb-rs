use types;
use ::Client;
use ql2::proto::Term;

pub trait IntoPrimaryKey {
    fn into_primary_key(self) -> Term;
}

impl<O> IntoPrimaryKey for Client<types::String, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for String {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoPrimaryKey for &'a String {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoPrimaryKey for &'a str {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoPrimaryKey for Client<types::Number, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for f32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for i32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for u32 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for f64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for i64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoPrimaryKey for u64 {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoPrimaryKey for Client<types::Bool, O> {
    fn into_primary_key(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoPrimaryKey for bool {
    fn into_primary_key(self) -> Term {
        Term::from_json(self)
    }
}
