#![feature(proc_macro)]

#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

mod args;


/// Splice an array of arguments into another term
///
/// `args` is a macro that’s used to splice a number of arguments into another term. This is
/// useful when you want to call a variadic term such as `branch` with a set of arguments produced at
/// runtime.
///
/// # Example
///
/// If `x` is greater than `5` return `big`, otherwise return `small`.
///
/// ```rust,ignore
/// r.branch(args!(r.expr(x).gt(5), "big", "small"));
/// ```
#[proc_macro]
pub fn args(input: TokenStream) -> TokenStream {
    args::Args::new(&input.to_string())
        .process().tokens
        .as_str()
        .parse()
        .unwrap()
}
