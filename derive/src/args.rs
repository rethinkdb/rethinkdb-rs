use {Args, KvPair};
use syn::{Expr, ExprClosure};
use proc_macro2::TokenStream;

impl Args {
    pub fn process(self, input: String) -> TokenStream {
        let mut body = TokenStream::new();

        let Args { elems, opts, closure } = self;

        if let Some(elems) = elems {
            process_elems(elems.0, &mut body);
        }

        if let Some(opts) = opts {
            process_opts(opts.0, &mut body);
        }

        if let Some(Expr::Closure(closure)) = closure {
            process_closure(closure, &mut body);
        }

        quote!({
            let mut args = Arg::new();
            args.set_string(&format!("args!({})", #input));
            #body
            args
        })
    }
}

fn process_elems(elems: impl IntoIterator<Item=Expr>, body: &mut TokenStream) {
    for expr in elems {
        if let Expr::Closure(closure) = expr {
            process_closure(closure, body);
            continue;
        }
        let arg = quote! {
            args.add_arg(#expr.into_arg());
        };
        body.extend(arg);
    }
}

fn process_opts(opts: impl IntoIterator<Item=KvPair>, body: &mut TokenStream) {
    let mut obj = quote! {
        let mut obj_val = Arg::new();
    };
    for KvPair(key, val) in opts {
        let key = key.to_string();
        let arg = quote! {
            match Arg::create_term_pair(#key, #val) {
                Ok(temp_pair) => obj_val.add_opt(temp_pair),
                Err(error) => obj_val.set_term(Err(error)),
            }
        };
        obj.extend(arg);
    }
    body.extend(obj);
    let arg = quote! {
        args.add_arg(obj_val.into_arg());
    };
    body.extend(arg);
}

fn process_closure(closure: ExprClosure, body: &mut TokenStream) {
    let mut args = TokenStream::new();
    for i in 0..closure.inputs.len() {
        args.extend(quote! {
            var!(#i),
        });
    }
    let closure = quote! {
        let func = func!((#closure), #args);
        args.add_arg(func.into_arg());
    };
    body.extend(closure);
}
