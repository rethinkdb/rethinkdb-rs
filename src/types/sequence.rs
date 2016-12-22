use ::Term;

pub trait IntoSequence {
    fn into_sequence(self) -> Term;
}
