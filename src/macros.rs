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
///
/// # Example
///
/// The `expr` macro can also compile an if else statement down to a ReQL branch command.
///
/// ```text
/// # #[macro_use] extern crate reql;
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # use reql::r;
/// # struct Value;
/// # fn main() {
/// let hero = r.table("marvel").get("Iron Man");
///
/// r.do_(expr!{
///
///     if hero.get_field("victories") > 100 {
///         hero.get_field("name") + " is a superhero"
///     }
///
///     else if hero.get_field("victories") > 10 {
///         hero.get_field("name") + " is a hero"
///     }
///
///     else {
///         hero.get_field("name") + " is very nice"
///     }
///
/// }).run::<String>();
/// # }
/// ```
#[macro_export]
macro_rules! expr {
    () => {};

    (trace if $($cond:tt)* { $($body:tt)* } $(else if $($elif_cond:tt)* { $($elif_body:tt)* })* else { $($el_body:tt)* } $($tail:tt)*) => {{
        expr!( r.branch(args!($($cond)*, $($body)*, $($($elif_cond)*, $($elif_body)*),* $($el_body)*)) $($tail)*)
    }};

    ( trace $($left:tt)* && $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).and($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* || $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).or($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* == $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).eq($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* != $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).ne($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* > $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).gt($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* >= $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).ge($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* < $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).lt($($right)*) $($tail)*)
    }};

    ( trace $($left:tt)* <= $($right:tt)* $($tail:tt)* ) => {{
        expr!( expr!($($left)*).le($($right)*) $($tail)*)
    }};

    ( trace ! $($cond:tt)* $($tail:tt)* ) => {{
        expr!( r.not($($cond)*) $($tail)*)
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
//// r.branch(args!(x > 5, "big", "small")).run::<String>();
/// r.branch(args!(r.table(x) > 5, "big", "small")).run::<String>();
/// # }
/// ```
///
/// # Related commands
///
/// * [arr](macro.arr.html)
#[macro_export]
macro_rules! args {
    (trace $($left:tt)* && $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).and($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* || $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).or($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* == $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).eq($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* != $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).ne($($right)*), $($tail)*)
    }};

    (trace $left:block > $right:block, $($tail:tt)* ) => {{
        args!(expr!($left).gt($right), $($tail)*)
    }};

    (trace $($left:tt)* >= $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).ge($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* < $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).lt($($right)*), $($tail)*)
    }};

    (trace $($left:tt)* <= $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).le($($right)*), $($tail)*)
    }};

    (trace ! $cond:tt, $($tail:tt)* ) => {{
        args!(r.not($cond), $($tail)*)
    }};

    (trace $($left:tt)* + $($right:tt)*, $($tail:tt)* ) => {{
        args!(expr!($($left)*).add($($right)*), $($tail)*)
    }};

    ($( $val:expr ),* $(,)*) => {{
        arr!($($val),*)
    }};
}

/// Take one or more values as arguments and return an array
#[macro_export]
macro_rules! arr {
    ($( $val:expr ),* $(,)*) => {{
        use $crate::ToArg;
        use $crate::Term;

        let mut term = Term::new();
        $(
            for arg in $val.to_arg() {
                term.mut_args().push(arg);
            }
        )*
        term
    }}
}

/// Create an object from a list of key-value pairs, where the keys must be strings
#[macro_export]
macro_rules! obj {
    ( $( $key:ident: $val:expr ),* $(,)*) => {{
        use $crate::ToArg;
        use $crate::Term;
        use $crate::TermPair;

        let mut object = Term::new();
        $(
            let mut term = Term::new();
            for arg in $val.to_arg() {
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
