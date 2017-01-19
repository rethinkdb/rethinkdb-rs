use {IntoArg, Command};
use types::FromJson;
use ql2::proto::Term;

impl IntoArg for Command {
    fn into_arg(self) -> Vec<Term> {
        vec![self.term.unwrap_or_else(|| Term::new())]
    }
}

impl IntoArg for String {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl<'a> IntoArg for &'a String {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl<'a> IntoArg for &'a str {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for f32 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for i32 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for u32 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for f64 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for i64 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for u64 {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for bool {
    fn into_arg(self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl IntoArg for Term {
    fn into_arg(self) -> Vec<Term> {
        vec![self]
    }
}
