#![recursion_limit = "128"]
#![feature(proc_macro_diagnostic)]

//! This crate provides macros for making ReQL types more pleasant to work with.
//! Currently, it only exposes the `args` macro but it's possible that more will
//! be added in the future.
//!
//! In this crate we make use of a couple nightly features, specifically
//! `proc_macro` and `proc_macro_non_items` so you will need to use the latest nightly
//! compiler in order to use the macros here. Add the following to your crate:-
//!
//! ```rust,ignore
//! #![feature(proc_macro, proc_macro_hygiene)]
//! ```

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
struct Opt(Punctuated<KvPair, Token![,]>);

#[derive(Debug, Clone)]
struct List(Punctuated<syn::Expr, Token![,]>);

#[derive(Debug, Clone)]
struct Args(Punctuated<Arg, Token![,]>);

#[derive(Debug, Clone)]
struct ToComma(proc_macro2::TokenStream);

#[derive(Debug, Clone)]
enum Arg {
    Expr(syn::Expr),
    Opt(Opt),
    Bad(ToComma),
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
        quote! { Term::new() }
    } else {
        args::process(input)
    };

    let expanded = quote!({
        #[allow(unused_imports)]
        use reql::{Term, IntoArg, Arg};
        #body
    });

    expanded.into()
}
