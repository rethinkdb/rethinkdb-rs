#![recursion_limit = "128"]
#![feature(proc_macro)]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

mod args;
mod parser;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;

#[derive(Debug, Clone)]
struct KvPair(syn::Ident, syn::Expr);

#[derive(Debug, Clone)]
struct Object(Punctuated<KvPair, Token![,]>);

#[derive(Debug, Clone)]
struct List(Punctuated<syn::Expr, Token![,]>);

#[derive(Debug, Clone)]
struct Elems(Punctuated<syn::Expr, Token![,]>);

#[derive(Debug, Clone)]
struct Args {
    elems: Option<Elems>,
    opts: Option<Object>,
    closure: Option<syn::Expr>,
}

/// Splice an array of arguments into another term
///
/// A macro that’s used to splice a number of arguments into another term. This is
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
    let body = if input.is_empty() {
        quote!(Term::new())
    }

    else {
        let raw = input.to_string();
        syn::parse::<Args>(input)
            .unwrap()
            .process(raw)
    };

    let expanded = quote!({
        #[allow(unused_imports)]
        use reql::{Term, IntoArg, Arg};
        #body
    });

    expanded.into()
}
