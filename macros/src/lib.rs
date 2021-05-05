extern crate proc_macro;

mod func;

use func::Func;
use proc_macro::TokenStream;

#[proc_macro]
pub fn func(input: TokenStream) -> TokenStream {
    Func::new(input.into()).process().into()
}
