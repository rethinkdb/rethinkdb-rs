use {Command, ToArg, Arg, Args};
use types::FromJson;
use ql2::proto::Term;
use serde_json::value::Value;

impl ToArg for Command {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.query.to_owned(),
            term: self.term.clone(),
        }
    }
}

impl ToArg for Args {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.string.to_owned(),
            term: self.term.clone(),
        }
    }
}

impl ToArg for Term {
    fn to_arg(&self) -> Arg {
        Arg {
            string: String::new(),
            term: self.clone(),
        }
    }
}

impl ToArg for String {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for char {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!("'{}'", self),
            term: Term::from_json(self),
        }
    }
}

impl<'a> ToArg for &'a String {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
        }
    }
}

impl<'a> ToArg for &'a str {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for f32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for i32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for u32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for f64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for i64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for u64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for bool {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}

impl ToArg for Value {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
        }
    }
}
