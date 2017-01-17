extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate case;

mod command;

use proc_macro::TokenStream;

#[proc_macro_derive(Command, attributes(command))]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let command = command::expand(&ast);
    command.parse().unwrap()
}
