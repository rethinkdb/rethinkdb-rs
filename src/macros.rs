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
        struct _DummyCommand;
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
/// # #![allow(unused_must_use)]
/// # #[macro_use] extern crate reql;
/// # use reql::commands::*;
/// # fn main() {
/// # let r = Command::new();
/// let x = 10;
/// r.branch(args!(r.expr(x).gt(5), "big", "small"));
/// # }
/// ```
#[macro_export]
macro_rules! args {
    ( $($arg:tt)+ ) => {{
        #[allow(unused_imports)]
        use $crate::{ToArg, Term};
        #[allow(unused_imports)]
        use $crate::commands::Args;

        let mut args = Args::new();
        {
            let term = args.mut_term();
            __process_args!(term, $($arg)+);
        }
        args
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __process_args {
    ( $term:ident, ) => { };
    
    ( $term:ident,  $(,)* { $($key:ident: $val:tt),* $(,)* } $(,)* ) => {{
        $(
            let key = stringify!($key);
            let mut val = Term::new();
            __process_args!(val, $val);
            let temp_pair = Args::create_term_pair(key, val);
            $term.mut_optargs().push(temp_pair);
         )*
    }};
    
    ( $term:ident,  $(,)* { $($key:ident: $val:tt),* $(,)* } $($tail:tt)* ) => {{
        let mut arg = Term::new();
        $(
            let key = stringify!($key);
            let mut val = Term::new();
            __process_args!(val, $val);
            let temp_pair = Args::create_term_pair(key, val);
            arg.mut_optargs().push(temp_pair);
         )*
        $term.mut_args().push(arg);
        __process_args!($term, $($tail)*);
    }};
    
    ( $term:ident,  $(,)* [ $($val:tt),* $(,)* ] $($tail:tt)* ) => {{
        let mut arg = Term::new();
        $(
            let mut val = Term::new();
            __process_args!(val, $val);
            arg.mut_args().push(val);
        )*
        $term.mut_args().push(arg);
        __process_args!($term, $($tail)*);
    }};
    
    ( $term:ident,  $(,)* | $($arg:ident),* $(,)* | { $($body:tt)+ } $($tail:tt)* ) => {{
        //unimplemented!();
        __process_args!($term, $($tail)*);
    }};
    
    ( $term:ident,  $(,)* | $($arg:ident),* $(,)* | $body:expr, $($tail:tt)+ ) => {{
        //unimplemented!();
        __process_args!($term, $($tail)+);
    }};
    
    ( $term:ident,  $(,)* | $($arg:ident),* $(,)* | $body:expr $(,)* ) => {{
        //unimplemented!();
    }};
    
    ( $term:ident,  $(,)* $arg:expr, $($tail:tt)+ ) => {{
        $term.mut_args().push($arg.to_arg());
        __process_args!($term, $($tail)+);
    }};
    
    ( $term:ident,  $(,)* $arg:expr $(,)* ) => {{
        $term.mut_args().push($arg.to_arg());
    }};
}
