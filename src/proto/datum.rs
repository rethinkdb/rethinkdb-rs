use ql2::datum::DatumType;
use ql2::term::TermType;
use ql2::{Datum, Term};

fn to_term(datum: Datum) -> Term {
    let mut term = Term::default();
    term.set_type(TermType::Datum);
    term.datum = Some(datum);
    term
}

pub fn r_bool(val: bool) -> Term {
    let mut datum = Datum::default();
    datum.set_type(DatumType::RBool);
    datum.r_bool = Some(val);
    to_term(datum)
}

pub fn r_num(val: f64) -> Term {
    let mut datum = Datum::default();
    datum.set_type(DatumType::RNum);
    datum.r_num = Some(val);
    to_term(datum)
}

pub fn r_str(val: String) -> Term {
    let mut datum = Datum::default();
    datum.set_type(DatumType::RStr);
    datum.r_str = Some(val);
    to_term(datum)
}
