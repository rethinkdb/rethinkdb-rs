pub mod connect;
pub mod db;
pub mod table;

use ql2::term::TermType;
use ql2::Term;

fn args(typ: TermType, terms: Vec<Term>) -> Term {
    let mut term = Term::default();
    term.set_type(typ);
    term.args = terms;
    term
}
