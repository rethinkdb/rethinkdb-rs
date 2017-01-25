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
/// # #[macro_use] extern crate reql;
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # fn main() {
/// # let conn = ();
/// # let r = Command::new();
/// let x = 10;
/// r.branch(args!(r.expr(x).gt(5), "big", "small")).run::<String>(&conn);
/// # }
/// ```
#[macro_export]
macro_rules! args {
    ( $($arg:tt)+ ) => {{
        use $crate::Term;
        use $crate::commands::Command;

        let mut term = Term::new();
        __process_args!(term, $($arg)+);
        let mut cmd = Command::new();
        cmd.set_term(term);
        cmd
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
            let temp_pair = Command::create_term_pair(key, val);
            $term.mut_optargs().push(temp_pair);
         )*
    }};
    
    ( $term:ident,  $(,)* { $($key:ident: $val:tt),* $(,)* } $($tail:tt)* ) => {{
        let mut arg = Term::new();
        $(
            let key = stringify!($key);
            let mut val = Term::new();
            __process_args!(val, $val);
            let temp_pair = Command::create_term_pair(key, val);
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
        #[allow(unused_imports)]
        use $crate::ToArg;
        $term.mut_args().push($arg.to_arg());
        __process_args!($term, $($tail)+);
    }};
    
    ( $term:ident,  $(,)* $arg:expr $(,)* ) => {{
        #[allow(unused_imports)]
        use $crate::ToArg;
        $term.mut_args().push($arg.to_arg());
    }};
}
