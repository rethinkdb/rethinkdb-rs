use {Client, Connection, ToArg, Arg, Args};
use types::FromJson;
use serde_json::value::Value;
use ql2::proto::{Term, Term_AssocPair as TermPair};
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;

impl ToArg for Client {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.query,
            term: self.term,
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for Args {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.string,
            term: self.term,
            pool: self.pool,
            remote: self.remote,
        }
    }
}

impl ToArg for Term {
    fn to_arg(self) -> Arg {
        Arg {
            string: String::new(),
            term: self,
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for String {
    fn to_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for char {
    fn to_arg(self) -> Arg {
        Arg {
            string: format!("'{}'", self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl<'a> ToArg for &'a String {
    fn to_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl<'a> ToArg for &'a str {
    fn to_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for f32 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for i32 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for u32 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for f64 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for i64 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for u64 {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for bool {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl ToArg for Value {
    fn to_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

#[cfg(feature = "with_io")]
impl ToArg for &'static Connection {
    fn to_arg(self) -> Arg {
        Arg {
            string: String::new(),
            term: Term::new(),
            pool: Some(*self),
            remote: None,
        }
    }
}

#[cfg(feature = "with_io")]
impl ToArg for Remote {
    fn to_arg(self) -> Arg {
        Arg {
            string: String::from("core.remote()"),
            term: Term::new(),
            pool: None,
            remote: Some(self),
        }
    }
}

impl Arg {
    /// Create a new command argument
    ///
    /// This is the return type of the `ToArg` trait. You need to
    /// use `Arg::new` to create an argument when implementing that
    /// trait for any additional types that you want to pass to ReQL
    /// commands.
    ///
    /// ReQL commands are represented as `term`s so you must first
    /// convert your argument to a term and pass it as `term` to this method.
    /// For debugging and logging purposes, this method also requires that you
    /// pass the string representation of your argument i.e. `as_str`.
    pub fn new(term: Term, as_str: &str) -> Arg {
        Arg {
            string: as_str.into(),
            term: term,
            pool: None,
            remote: None,
        }
    }

    #[doc(hidden)]
    pub fn term(self) -> Term {
        self.term
    }

    #[doc(hidden)]
    pub fn pool(self) -> Option<Connection> {
        self.pool
    }
}

impl Args {
    #[doc(hidden)]
    pub fn new() -> Args {
        Args {
            term: Term::new(),
            string: String::new(),
            pool: None,
            remote: None,
        }
    }

    #[doc(hidden)]
    pub fn term(&self) -> &Term {
        &self.term
    }

    #[doc(hidden)]
    pub fn mut_term(&mut self) -> &mut Term {
        &mut self.term
    }

    #[doc(hidden)]
    pub fn set_term(&mut self, term: Term) {
        self.term = term;
    }

    #[doc(hidden)]
    pub fn add_arg(&mut self, arg: Arg) {
        self.pool = arg.pool;
        self.remote = arg.remote;
        self.term.mut_args().push(arg.term);
    }

    #[doc(hidden)]
    pub fn set_string(&mut self, string: String) {
        self.string = string;
    }

    #[doc(hidden)]
    pub fn create_term_pair<T: ::ToArg>(key: &str, val: T) -> TermPair {
        let mut temp = Term::new();
        temp.mut_args().push(val.to_arg().term);
        let mut temp_pair = TermPair::new();
        temp_pair.set_key(key.into());
        temp_pair.set_val(temp);
        temp_pair
    }
}
