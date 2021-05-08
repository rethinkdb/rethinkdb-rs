extern crate proc_macro;

mod func;
mod options;

use func::Func;
use proc_macro::TokenStream;

#[proc_macro]
pub fn func(input: TokenStream) -> TokenStream {
    Func::new(input.into()).process().into()
}

#[proc_macro_derive(CommandOptions)]
pub fn command_opts(input: TokenStream) -> TokenStream {
    options::parse(input)
}
