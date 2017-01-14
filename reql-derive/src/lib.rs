extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod command;

use proc_macro::TokenStream;

#[proc_macro_derive(Command)]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let command = command::expand(&ast);
    command.parse().unwrap()
}

/*
fn to_titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
*/
