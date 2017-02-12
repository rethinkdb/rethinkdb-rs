macro_rules! with_args {
    ( $cmd:ident, $args:ident ) => {{
        let term = $cmd.mut_term();
        let mut tmp_args = $args.term;
        if tmp_args.has_field_type() { // did not come from the args macro
            term.mut_args().push(tmp_args);
        } else { // came from the args macro
            for arg in tmp_args.take_args().into_vec() {
                term.mut_args().push(arg);
            }
            for pair in tmp_args.take_optargs().into_vec() {
                term.mut_optargs().push(pair);
            }
        }
    }}
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
/// # use reql::Client;
/// # use reql::commands::*;
/// # fn main() {
/// # let r = Client::new();
/// let x = 10;
/// r.branch(args!(r.expr(x).gt(5), "big", "small"));
/// # }
/// ```
#[macro_export]
macro_rules! args {
    ( $($arg:tt)+ ) => {{
        #[allow(unused_imports)]
        use $crate::{ToArg, Args, Term};

        let mut args = Args::new();
        let string = stringify!($($arg)+);
        args.set_string(format!("args!({})", string));
        __process_args!(args, $($arg)+);
        args
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __process_args {
    ( $args:ident, ) => { };
    
    ( $args:ident,  $(,)* { $($key:ident: $val:tt),* $(,)* } $(,)* ) => {{
        $(
            let key = stringify!($key);
            let mut val = Args::new();
            __process_args!(val, $val);
            let temp_pair = Args::create_term_pair(key, val);
            $args.mut_term().mut_optargs().push(temp_pair);
         )*
    }};
    
    ( $args:ident,  $(,)* { $($key:ident: $val:tt),* $(,)* } $($tail:tt)* ) => {{
        let mut arg = Term::new();
        $(
            let key = stringify!($key);
            let mut val = Args::new();
            __process_args!(val, $val);
            let temp_pair = Args::create_term_pair(key, val);
            arg.mut_optargs().push(temp_pair);
         )*
        $args.mut_term().mut_args().push(arg);
        __process_args!($args, $($tail)*);
    }};
    
    ( $args:ident,  $(,)* [ $($val:tt),* $(,)* ] $($tail:tt)* ) => {{
        let mut arg = Term::new();
        $(
            let mut val = Args::new();
            __process_args!(val, $val);
            arg.mut_args().push(val.to_arg().term());
        )*
        $args.mut_term().mut_args().push(arg);
        __process_args!($args, $($tail)*);
    }};
    
    ( $args:ident,  $(,)* move | $($arg:ident),* $(,)* | { $($body:tt)+ } $($tail:tt)* ) => {{
        //unimplemented!();
        __process_args!($args, $($tail)*);
    }};
    
    ( $args:ident,  $(,)* move | $($arg:ident),* $(,)* | $body:expr, $($tail:tt)+ ) => {{
        //unimplemented!();
        __process_args!($args, $($tail)+);
    }};
    
    ( $args:ident,  $(,)* move | $($arg:ident),* $(,)* | $body:expr $(,)* ) => {{
        //unimplemented!();
    }};
    
    ( $args:ident,  $(,)* | $($arg:ident),* $(,)* | { $($body:tt)+ } $($tail:tt)* ) => {{
        //unimplemented!();
        __process_args!($args, $($tail)*);
    }};
    
    ( $args:ident,  $(,)* | $($arg:ident),* $(,)* | $body:expr, $($tail:tt)+ ) => {{
        //unimplemented!();
        __process_args!($args, $($tail)+);
    }};
    
    ( $args:ident,  $(,)* | $($arg:ident),* $(,)* | $body:expr $(,)* ) => {{
        //unimplemented!();
    }};
    
    ( $args:ident,  $(,)* $arg:expr, $($tail:tt)+ ) => {{
        $args.add_arg($arg.to_arg());
        __process_args!($args, $($tail)+);
    }};
    
    ( $args:ident,  $(,)* $arg:expr $(,)* ) => {{
        $args.add_arg($arg.to_arg());
    }};
}
