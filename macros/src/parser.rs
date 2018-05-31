use {KvPair, Opt, List, Arg, Args, ToComma};
use syn::synom::Synom;
use syn::{Ident, Expr};
use syn::punctuated::Punctuated;
use proc_macro2::{TokenTree, TokenStream};
use syn::buffer::Cursor;
use syn::synom::PResult;
use std::iter::FromIterator;

impl Synom for KvPair {
    named!(parse -> Self, do_parse!(
            key: syn!(Ident) >>
            punct!(:) >>
            val: syn!(Expr) >>
            (KvPair(key, val))
    ));

    fn description() -> Option<&'static str> {
        Some("key value pair")
    }
}

impl Synom for Opt {
    named!(parse -> Self, do_parse!(
            body: braces!(call!(Punctuated::parse_terminated_nonempty)) >>
            (Opt(body.1))
    ));

    fn description() -> Option<&'static str> {
        Some("opt")
    }
}

impl Synom for List {
    named!(parse -> Self, do_parse!(
            body: brackets!(call!(Punctuated::parse_terminated_nonempty)) >>
            (List(body.1))
    ));

    fn description() -> Option<&'static str> {
        Some("list")
    }
}

impl Synom for Args {
    named!(parse -> Self, do_parse!(
            body: call!(Punctuated::parse_terminated_nonempty) >>
            (Args(body))
    ));

    fn description() -> Option<&'static str> {
        Some("arguments")
    }
}

impl Synom for Arg {
    fn parse(input: Cursor) -> PResult<Self> {
        // is it an expression?
        if let Ok((expr, cursor)) = Expr::parse(input) {
            if cursor.eof() {
                return Ok((Arg::Expr(expr), cursor));
            }
            if let Some((tree, _)) = cursor.token_tree() {
                if let TokenTree::Punct(p) = tree {
                    if p.as_char() == ',' {
                        return Ok((Arg::Expr(expr), cursor));
                    }
                }
            }
        }
        // how about an option?
        if let Ok((opt, cursor)) = Opt::parse(input) {
            return Ok((Arg::Opt(opt), cursor));
        }
        // it must be a bad argument then
        if let Ok((to_comma, cursor)) = ToComma::parse(input) {
            return Ok((Arg::Bad(to_comma), cursor));
        }
        reject!("unexpected input",)
    }

    fn description() -> Option<&'static str> {
        Some("argument")
    }
}

impl Synom for ToComma {
    fn parse(mut rest: Cursor) -> PResult<Self> {
        let mut tokens = Vec::new();
        while let Some((tree, cursor)) = rest.token_tree() {
            if let TokenTree::Punct(p) = tree.clone() {
                if p.as_char() == ',' {
                    if !tokens.is_empty() {
                        break;
                    }
                }
            }
            rest = cursor;
            tokens.push(tree);
        }
        let stream = TokenStream::from_iter(tokens);
        Ok((ToComma(stream), rest))
    }
    
    fn description() -> Option<&'static str> {
        Some("token stream to comma")
    }
}
