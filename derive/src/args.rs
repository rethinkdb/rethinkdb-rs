use {Args, KvPair};
use quote::ToTokens;
use proc_macro2::TokenStream;
use syn::{self, Expr, ExprClosure, FnArg, ArgCaptured};

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
        body.extend(quote! {
            args.add_arg(#expr.into_arg());
        });
    }
}

fn _process_list(exprs: impl IntoIterator<Item=Expr>, body: &mut TokenStream) {
    let mut arg = quote! {
        let mut list_arg = Arg::new();
    };
    for expr in exprs {
        arg.extend(quote! {
            list_arg.add_arg(#expr.into_arg());
        });
    }
    arg.extend(quote! {
        args.add_arg(list_arg.into_arg());
    });
    body.extend(arg);
}

fn process_opts(opts: impl IntoIterator<Item=KvPair>, body: &mut TokenStream) {
    for KvPair(key, val) in opts {
        let key = key.to_string();
        body.extend(quote! {
            match Arg::create_term_pair(#key, #val) {
                Ok(temp_pair) => args.add_opt(temp_pair),
                Err(error) => args.set_term(Err(error)),
            }
        });
    }
}

fn process_closure(mut closure: ExprClosure, body: &mut TokenStream) {
    let mut args = TokenStream::new();
    for (i, mut arg) in closure.inputs.iter_mut().enumerate() {
        args.extend(quote! {
            var!(#i),
        });
        match arg {
            FnArg::Inferred(pat) => {
                let captured = ArgCaptured {
                    pat: pat.clone(),
                    colon_token: Default::default(),
                    ty: syn::parse_str("Client").unwrap(),
                };
                *arg = FnArg::Captured(captured);
            }
            arg => {
                let arg = arg.clone().into_token_stream().to_string();
                panic!(format!("`{}`: type annotations are not supported in ReQL closure arguments", arg.replace(" :", ":")));
            }
        }
    }
    body.extend(quote! {
        let func = func!((#closure), #args);
        args.add_arg(func.into_arg());
    });
}
