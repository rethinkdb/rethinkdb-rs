#![feature(proc_macro)]

#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

mod args;

#[proc_macro]
pub fn args_impl(input: TokenStream) -> TokenStream {
    args::Args::new(&input.to_string())
        .process().tokens
        .as_str()
        .parse()
        .unwrap()
}
