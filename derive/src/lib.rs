#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate quote;

mod args;

proc_macro_expr_impl! {
    pub fn args_impl(input: &str) -> String {
        args::Args::new(input)
            .process().tokens
            .to_string()
    }
}
