use {Args, KvPair};
use proc_macro2::TokenStream;
use syn::token::Comma;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
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
    let mut ids = quote! {
        #[allow(unused_mut)]
        let mut ids = Vec::new();
    };
    let mut args = Punctuated::new();
    for (i, mut arg) in closure.inputs.iter_mut().enumerate() {
        let var = var(i);
        ids.extend(quote! {
            for t in #var.term().unwrap().get_args() {
                ids.push(t.get_datum().clone());
            }
        });
        args.push(syn::parse(var.into()).unwrap());
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
                arg.span().unstable()
                    .error("unsupported ReQL closure argument")
                    .note("only arguments with no type annotations are supported")
                    .emit();
            }
        }
    }
    ids.extend(quote!(ids));
    let func = func(closure, args, ids);
    body.extend(quote! {
        use reql::{Client, RepeatedField, Term, Datum, TT, DT};

        args.add_arg(#func.into_arg());
    });
}

fn func(closure: ExprClosure, args: Punctuated<Expr, Comma>, ids: TokenStream) -> TokenStream {
    quote! {{
        let res: Client = (#closure)(#args);
        let mut closure = Client::new();
        match res.term() {
            Ok(res) => {
                // ARRAY
                let mut array = Datum::new();
                array.set_field_type(DT::R_ARRAY);
                let args = RepeatedField::from_vec({#ids});
                array.set_r_array(args);
                // DATUM
                let mut datum = Term::new();
                datum.set_field_type(TT::DATUM);
                datum.set_datum(array);
                // FUNC
                let mut func = Term::new();
                func.set_field_type(TT::FUNC);
                let args = RepeatedField::from_vec(vec![datum, res.clone()]);
                func.set_args(args);
                closure.set_term(Ok(func));
            }
            Err(error) => {
                closure.set_term(Err(error));
            }
        }
        closure
    }}
}

fn var(idx: usize) -> TokenStream {
    quote! {{
        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num(#idx as f64);
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(id);
        // VAR
        let mut var = Term::new();
        var.set_field_type(TT::VAR);
        let args = RepeatedField::from_vec(vec![datum]);
        var.set_args(args);
        let mut client = Client::new();
        client.set_term(Ok(var));
        client
    }}
}
