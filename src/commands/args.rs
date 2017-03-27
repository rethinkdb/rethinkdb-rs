use {Client, Connection, QueryError, IntoArg, Arg, Args};
use types::FromJson;
use serde_json::value::Value;
use ql2::proto::{Term, Term_AssocPair as TermPair};
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;

impl IntoArg for Client {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.query,
            term: self.term,
            error: self.error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for Args {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.string,
            term: self.term,
            error: self.error,
            pool: self.pool,
            remote: self.remote,
        }
    }
}

impl IntoArg for Term {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::new(),
            term: self,
            error: QueryError::None,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for String {
    fn into_arg(self) -> Arg {
        let string = format!(r#""{}""#, self);
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: string,
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for char {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: format!("'{}'", self),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl<'a> IntoArg for &'a String {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: format!(r#""{}""#, self),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl<'a> IntoArg for &'a str {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: format!(r#""{}""#, self),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for f32 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for i32 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for u32 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for f64 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for i64 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for u64 {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for bool {
    fn into_arg(self) -> Arg {
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: self.to_string(),
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for Value {
    fn into_arg(self) -> Arg {
        let string = self.to_string();
        let (term, error) = match Term::from_json(self) {
            Ok(term) => (term, QueryError::None),
            Err(err) => (Term::new(), QueryError::from(err)),
        };

        Arg {
            string: string,
            term: term,
            error: error,
            pool: None,
            remote: None,
        }
    }
}

#[cfg(feature = "with_io")]
impl IntoArg for &'static Connection {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::new(),
            term: Term::new(),
            error: QueryError::None,
            pool: Some(*self),
            remote: None,
        }
    }
}

#[cfg(feature = "with_io")]
impl IntoArg for Remote {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::from("core.remote()"),
            term: Term::new(),
            error: QueryError::None,
            pool: None,
            remote: Some(self),
        }
    }
}

impl Arg {
    /// Create a new command argument
    ///
    /// This is the return type of the `IntoArg` trait. You need to
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
            error: QueryError::None,
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
            error: QueryError::None,
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
    pub fn set_string(&mut self, string: &str) {
        self.string = string.into();
    }

    #[doc(hidden)]
    pub fn create_term_pair<T: ::IntoArg>(key: &str, val: T) -> TermPair {
        let mut temp = Term::new();
        temp.mut_args().push(val.into_arg().term);
        let mut temp_pair = TermPair::new();
        temp_pair.set_key(key.into());
        temp_pair.set_val(temp);
        temp_pair
    }
}
