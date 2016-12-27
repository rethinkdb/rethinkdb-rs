use {Client, Term, types};
use serde_json::value::ToJson;

pub trait IntoTerm {
    fn into_term(self) -> Term;
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

impl IntoTerm for bool {
    fn into_term(self) -> Term {
        Term::from_json(self)
    }
}

impl IntoTerm for Term {
    fn into_term(self) -> Term {
        self
    }
}

impl<T, O> IntoTerm for Client<T, O>
    where T: types::DataType,
          O: ToJson + Clone
{
    fn into_term(self) -> Term {
        self.into()
    }
}
