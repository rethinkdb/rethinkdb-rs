use {ToArg, Command};
use types::FromJson;
use ql2::proto::Term;

impl ToArg for Command {
    fn to_arg(&self) -> Vec<Term> {
        vec![self.term.clone()]
    }
}

impl ToArg for String {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl<'a> ToArg for &'a String {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl<'a> ToArg for &'a str {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for f32 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for i32 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for u32 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for f64 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for i64 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for u64 {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for bool {
    fn to_arg(&self) -> Vec<Term> {
        vec![Term::from_json(self)]
    }
}

impl ToArg for Term {
    fn to_arg(&self) -> Vec<Term> {
        vec![self.clone()]
    }
}

impl ToArg for Vec<Term> {
    fn to_arg(&self) -> Vec<Term> {
        self.clone()
    }
}
