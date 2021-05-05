#[cfg(test)]
mod tests;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::iter::FromIterator;
use syn::Ident;

#[derive(Debug)]
pub(super) struct Func {
    mv: Option<Ident>,
    args: Vec<Ident>,
    body: TokenStream,
}

impl Func {
    pub(super) fn new(input: TokenStream) -> Self {
        let mut iter = input.into_iter();
        let mv = iter.next().map(first).unwrap();
        if mv.is_some() {
            iter.next().filter(is_pipe).unwrap();
        }
        let mut args = Vec::new();
        while let Some(token) = iter.next() {
            if is_pipe(&token) {
                break;
            }
            args.push(ident(token));
            let token = iter.next().unwrap();
            if is_pipe(&token) {
                break;
            }
            assert_comma(&token);
        }
        let body = TokenStream::from_iter(iter);
        Self { mv, args, body }
    }

    pub(super) fn process(self) -> TokenStream {
        let Self { mv, args, body } = self;
        let mut header = quote!(#mv |);
        let mut params = TokenStream::new();
        let func_args = args.len();
        for (i, arg) in args.into_iter().enumerate() {
            let var = quote!(reql::Query::var(*ids.get(#i).unwrap()));
            if i == func_args - 1 {
                header.extend(quote!(#arg: reql::Query));
                params.extend(quote!(#var));
            } else {
                header.extend(quote!(#arg: reql::Query, ));
                params.extend(quote!(#var, ));
            }
        }
        header.extend(quote!(|));
        let closure = quote!(#header #body);
        quote!(reql::Func({
            let closure = #closure;
            let mut ids = Vec::with_capacity(#func_args);
            for _ in 0..#func_args {
                let id = reql::var_counter();
                ids.push(id);
            }
            let func = closure(#params);
            let args = reql::Query::from_json(ids);
            reql::Query::new(reql::TermType::Func).with_arg(args).with_arg(func)
        }))
    }
}

fn first(token: TokenTree) -> Option<Ident> {
    if is_pipe(&token) {
        return None;
    }
    match token {
        TokenTree::Ident(ident) if ident == "move" => Some(ident),
        _ => panic!("invalid closure"),
    }
}

fn ident(token: TokenTree) -> Ident {
    match token {
        TokenTree::Ident(ident) => ident,
        _ => panic!("invalid closure"),
    }
}

fn is_pipe(token: &TokenTree) -> bool {
    matches!(token, TokenTree::Punct(punct) if punct.as_char() == '|')
}

fn assert_comma(token: &TokenTree) {
    match token {
        TokenTree::Punct(punct) if punct.as_char() == ',' => {}
        _ => panic!("invalid closure"),
    }
}
