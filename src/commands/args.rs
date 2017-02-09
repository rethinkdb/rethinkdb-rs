use {Client, Pool, ToArg, Arg, Args};
use types::FromJson;
use slog::Logger;
use serde_json::value::Value;
use ql2::proto::{Term, Term_AssocPair as TermPair};

impl ToArg for Client {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.query.to_owned(),
            term: self.term.clone(),
            pool: None,
        }
    }
}

impl ToArg for Args {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.string.to_owned(),
            term: self.term.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl ToArg for Term {
    fn to_arg(&self) -> Arg {
        Arg {
            string: String::new(),
            term: self.clone(),
            pool: None,
        }
    }
}

impl ToArg for String {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for char {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!("'{}'", self),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl<'a> ToArg for &'a String {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl<'a> ToArg for &'a str {
    fn to_arg(&self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for f32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for i32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for u32 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for f64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for i64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for u64 {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for bool {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl ToArg for Value {
    fn to_arg(&self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
        }
    }
}

impl<'a> ToArg for &'a Pool {
    fn to_arg(&self) -> Arg {
        let pool = self.clone().clone();
        Arg {
            string: String::new(),
            term: Term::new(),
            pool: Some(pool),
        }
    }
}

impl Arg {
    #[doc(hidden)]
    pub fn term(self) -> Term {
        self.term
    }

    #[doc(hidden)]
    pub fn pool(self) -> Option<Pool> {
        self.pool
    }
}

impl Client {
    /// Creates a new command
    ///
    /// This is typically called `r`.
    ///
    /// # Example
    ///
    /// ```
    /// # #![allow(unused_must_use)]
    /// # extern crate reql;
    /// # use reql::Client;
    /// # use reql::commands::*;
    /// # fn main() {
    /// let r = Client::new();
    /// r.table("users");
    /// # }
    /// ```
    pub fn new() -> Client {
        Client {
            term: Term::new(),
            query: String::from("r"),
            logger: Logger::root(::slog::Discard, o!()),
        }
    }

    /// Sets a logger
    pub fn with_logger(&self, logger: Logger) -> Client {
        let mut cmd = self.clone();
        cmd.logger = logger;
        cmd
    }

    pub fn query(&self) -> &str {
        &self.query
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
}

impl Args {
    #[doc(hidden)]
    pub fn new() -> Args {
        Args {
            term: Term::new(),
            string: String::new(),
            pool: None,
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
