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
#[macro_export]
macro_rules! args {
    () => {};

    (trace $($left:tt)* && $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).and($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* || $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).or($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* == $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).eq($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* != $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).ne($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* > $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).gt($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* >= $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).ge($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* < $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).lt($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* <= $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).le($($right)*), $($tail)*)
    }};

    (trace ! $cond:tt, $($tail:tt)* ) => {{
        args!(r.not($cond), $($tail)*)
    }};

    (trace $($left:tt)* + $($right:tt)*, $($tail:tt)* ) => {{
        args!(args!($($left)*).add($($right)*), $($tail)*)
    }};

    ( trace { $( $key:ident: $($val:tt)+ ),* } $(,)* $($tail:tt)* ) => {{
        use $crate::{ToArg, Term, TermPair};

        let mut object = Term::new();
        $(
            let mut term = Term::new();
            for arg in args!($($val)+).to_arg() {
                term.mut_args().push(arg);
            }
            let key = stringify!($key);
            let mut term_pair = TermPair::new();
            term_pair.set_key(key.into());
            term_pair.set_val(term);
            object.mut_optargs().push(term_pair);
         )*
        args!(args!(object), $($tail)*)
    }};

    ( trace [ $( $val:expr ),* $(,)* ] $(,)* $($tail:tt)* ) => {{
        use $crate::{ToArg, Term};

        let mut term = Term::new();
        $(
            for arg in $val.to_arg() {
                term.mut_args().push(arg);
            }
        )*
        args!(args!(term), $($tail)*)
    }};

    ($( $val:expr ),* $(,)*) => {{
        use $crate::{ToArg, Term, Command};

        let mut term = Term::new();
        $(
            for arg in $val.to_arg() {
                term.mut_args().push(arg);
            }
        )*
        Command::new(term)
    }};
}
