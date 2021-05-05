use super::*;

#[test]
fn with_move() {
    Func::new(quote!(move |doc| {
        doc.get_field("author").bracket("name")
    }))
    .process();
}

#[test]
fn with_no_arg() {
    Func::new(quote!(|| r.expr("Hello world!"))).process();
}

#[test]
fn with_one_arg() {
    Func::new(quote!(|doc| { doc.get_field("author").bracket("name") })).process();
}

#[test]
fn with_multiple_args() {
    Func::new(quote!(|with, multiple, args| r.expr(with, multiple, args))).process();
}
