use ::{Client, Term};
use types::data;

pub trait IntoTerm {
    fn into_term(self) -> Term;
}

impl<O> IntoTerm for Client<data::String, O> {
    fn into_term(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoTerm for String {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoTerm for &'a String {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<'a> IntoTerm for &'a str {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoTerm for Client<data::Number, O> {
    fn into_term(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoTerm for f32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for i32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for u32 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for f64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for i64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for u64 {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl<O> IntoTerm for Client<data::Bool, O> {
    fn into_term(self) -> Term {
        self.cmd.0.into()
    }
}

impl IntoTerm for bool {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}
