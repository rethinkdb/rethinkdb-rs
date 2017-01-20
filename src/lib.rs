//! A native RethinkDB driver written in Rust

// `expr` macro recurses deeply

// Currently can't set these within lazy_static
// These are for `r`
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

#[macro_use]
extern crate reql_derive;
extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

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
    term: Term,
}

lazy_static! {
    /// The top-level ReQL namespace
    pub static ref r: Command = Command {
        term: Term::new(),
    };
}

/// The argument that is passed to any ReQL command
pub trait IntoArg {
    fn into_arg(self) -> Vec<Term>;
}

impl Command {
    pub fn new(term: Term) -> Command {
        Command {
            term: term,
        }
    }
}

/// Construct a ReQL JSON object from a Rust expression
///
/// # Example
///
/// Objects wrapped with expr can then be manipulated by ReQL API functions.
///
/// ```
/// # #[macro_use] extern crate reql;
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # use reql::r;
/// # struct Value;
/// # fn main() {
/// expr!(obj!{a: "b"}).merge(obj!{b: arr![1, 2, 3]}).run::<Value>();
/// # }
/// ```
#[macro_export]
macro_rules! expr {
    () => {};

    ( trace $left:tt && $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).and($right) $($tail)*)
    }};

    ( trace $left:tt || $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).or($right) $($tail)*)
    }};

    ( trace $left:tt == $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).eq($right) $($tail)*)
    }};

    ( trace $left:tt != $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).ne($right) $($tail)*)
    }};

    ( trace $left:tt > $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).gt($right) $($tail)*)
    }};

    ( trace $left:tt >= $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).ge($right) $($tail)*)
    }};

    ( trace $left:tt < $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).lt($right) $($tail)*)
    }};

    ( trace $left:tt <= $right:tt $($tail:tt)* ) => {{
        expr!( expr!($left).le($right) $($tail)*)
    }};

    ( trace ! $cond:tt $($tail:tt)* ) => {{
        expr!( r.not($cond) $($tail)*)
    }};

    ($( $val:expr ),* $(,)*) => {{
        $crate::Command::new(arr!($($val),*))
    }};
}

/// Splice an array of arguments into another term
///
/// `args` is a macro that’s used to splice a number of arguments into another term. This is
/// useful when you want to call a variadic term such as [branch](commands/trait.Branch.html) with a set of arguments produced at
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
/// let x = 10;
/// r.branch(args!(x > 5, "big", "small")).run::<String>();
/// # }
/// ```
///
/// # Related commands
///
/// * [arr](macro.arr.html)
#[macro_export]
macro_rules! args {
    ( $left:tt && $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).and($right), $($rest),*)
    }};

    ( $left:tt || $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).or($right), $($rest),*)
    }};

    ( $left:tt == $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).eq($right), $($rest),*)
    }};

    ( $left:tt != $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).ne($right), $($rest),*)
    }};

    ( $left:tt > $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).gt($right), $($rest),*)
    }};

    ( $left:tt >= $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).ge($right), $($rest),*)
    }};

    ( $left:tt < $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).lt($right), $($rest),*)
    }};

    ( $left:tt <= $right:tt, $($rest:expr),* $(,)* ) => {{
        args!(expr!($left).le($right), $($rest),*)
    }};

    ( ! $cond:tt, $($rest:expr),* $(,)* ) => {{
        args!(r.not($cond), $($rest),*)
    }};

    ($( $val:expr ),* $(,)*) => {{
        arr!($($val),*)
    }};
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
