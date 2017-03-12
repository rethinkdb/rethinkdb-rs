#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate quote;

use quote::Tokens;

proc_macro_expr_impl! {
    pub fn args_impl(input: &str) -> String {
        if input.trim().is_empty() {
            return quote!(reql::Term::new()).to_string();
        }

        let body = process_args(input);
        let args = format!("args!({})", input);

        quote!({
            let mut args = reql::Args::new();
            args.set_string(#args);
            #body
            args
        }).to_string()
    }
}

fn process_args(input: &str) -> Tokens {
    Tokens::new()
}
