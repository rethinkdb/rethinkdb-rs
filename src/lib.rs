//! A native RethinkDB driver written in Rust

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;

#[cfg(test)]
mod tests;

macro_rules! commands {
    ($($cmd:ident),* $(,)*) => {
        $(
            mod $cmd;
            pub use self::$cmd::*;
        )*
    }
}

macro_rules! command {
    ( $(#[$attr:meta])* ) => {
        #[derive(Command)]
        $(#[$attr])*
        struct _Dummy;
    }
}

pub mod commands;
mod types;
mod args;

#[doc(hidden)]
pub use ql2::proto::{Term, Term_AssocPair as TermPair};

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Command {
    term: Option<Term>,
    idx: u32,
}

/// The top-level ReQL namespace
#[allow(non_upper_case_globals)]
pub const r: Command = Command {
    term: None,
    idx: 0,
};

/// The argument that is passed to any ReQL command
pub trait IntoArg {
    fn into_arg(self) -> Vec<Term>;
}

/// Splice an array of arguments into another term
///
/// `args` is a macro that’s used to splice a number of arguments into another term. This is
/// useful when you want to call a variadic term such as [get_all](commands/trait.GetAll.html) with a set of arguments produced at
/// runtime.
///
/// *Note* that args evaluates all its arguments before passing them into the parent term, even if
/// the parent term otherwise allows lazy evaluation.
///
/// # Example
///
/// Get Alice and Bob from the table `people`.
///
/// ```
/// # #[macro_use] extern crate reql;
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # use reql::r;
/// # fn main() {
/// let (x, y) = (10, 5);
/// r.branch(args!(x > y, "big", "small")).run::<String>();
/// # }
/// ```
///
/// # Related commands
///
/// * [arr](macro.arr.html)
#[macro_export]
macro_rules! args {
    ( $left:ident > $right:ident, $($rest:expr),* $(,)* ) => {{
        args!(r.expr($left).gt($right), $($rest),*)
    }};

    ($( $val:expr ),* $(,)*) => {{
        arr!($($val),*)
    }}
}

/// Take one or more values as arguments and return an array
#[macro_export]
macro_rules! arr {
    ($( $val:expr ),* $(,)*) => {{
        use $crate::IntoArg;
        use $crate::Term;

        let mut term = Term::new();
        $(
            for arg in $val.into_arg() {
                term.mut_args().push(arg);
            }
        )*
        term
    }}
}

/// Create an object from a list of key-value pairs, where the keys must be strings
#[macro_export]
macro_rules! obj {
    ($( $key:ident: $val:expr ),* $(,)*) => {{
        use $crate::IntoArg;
        use $crate::Term;
        use $crate::TermPair;

        let mut object = Term::new();
        $(
            let mut term = Term::new();
            for arg in $val.into_arg() {
                term.mut_args().push(arg);
            }
            let key = stringify!($key);
            let mut term_pair = TermPair::new();
            term_pair.set_key(key.into());
            term_pair.set_val(term);
            object.mut_optargs().push(term_pair);
         )*
        object
    }}
}
