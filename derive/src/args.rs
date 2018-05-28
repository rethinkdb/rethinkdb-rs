use {Args, KvPair};
use proc_macro2::TokenStream;

impl Args {
    pub fn process(self, input: String) -> TokenStream {
        let args = format!("args!({})", input);
        let mut body = TokenStream::empty();

        let Args { elems, opts, closure } = self;

        if let Some(elems) = elems {
            for expr in elems.0 {
                let arg = quote! {
                    args.add_arg(#expr.into_arg());
                };
                body.extend(arg);
            }
        }

        if let Some(opts) = opts {
            let mut obj = quote! {
                let mut obj_val = Arg::new();
            };
            for KvPair(key, val) in opts.0 {
                let arg = quote! {
                    match Arg::create_term_pair(#key, #val) {
                        Ok(temp_pair) => obj_val.add_opt(temp_pair),
                        Err(error) => obj_val.set_term(Err(error)),
                    }
                };
                obj.extend(arg);
            }
            body.extend(obj);
        }

        if let Some(_closure) = closure {
            unimplemented!();
        }

        quote!({
            let mut args = Arg::new();
            args.set_string(#args);
            #body
            args
        })
    }
}
