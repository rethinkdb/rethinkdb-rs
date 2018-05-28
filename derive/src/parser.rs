use {KvPair, Object, List, Args, Elems};
use syn::synom::Synom;
use syn::{Ident, Expr};
use syn::punctuated::Punctuated;

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

impl Synom for Object {
    named!(parse -> Self, do_parse!(
            body: braces!(call!(Punctuated::parse_terminated_nonempty)) >>
            (Object(body.1))
    ));

    fn description() -> Option<&'static str> {
        Some("object")
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

impl Synom for Elems {
    named!(parse -> Self, do_parse!(
            body: call!(Punctuated::parse_terminated_nonempty) >>
            (Elems(body))
    ));

    fn description() -> Option<&'static str> {
        Some("elements")
    }
}

impl Synom for Args {
    named!(parse -> Self, do_parse!(
            elems: option!(syn!(Elems)) >>
            opts: option!(syn!(Object)) >>
            closure: option!(syn!(Expr)) >>
            (Args { elems, opts, closure })
    ));

    fn description() -> Option<&'static str> {
        Some("arguments")
    }
}
