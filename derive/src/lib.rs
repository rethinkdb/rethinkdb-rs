#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    pub fn args_impl(input: &str) -> String {
        let mut args = String::new();

        if input.trim().is_empty() {
            args.push_str("reql::Term::new()");
            return args;
        }

        args.push_str(&format!(r#"
            {{
                let mut args = reql::Args::new();
                args.set_string("args!({})");
                {}
                args
            }}
        "#, input.replace(r#"""#, r#"\""#), process_args(input)));

        args
    }
}

fn process_args(input: &str) -> String {
    String::new()
}
