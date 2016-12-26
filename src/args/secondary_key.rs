use types;
use ::{Client, Term};
use args::primary_key::IntoPrimaryKey;

pub trait IntoSecondaryKey : IntoPrimaryKey where Self: Sized {
    fn into_secondary_key(self) -> Term {
        self.into_primary_key()
    }
}

impl<O> IntoSecondaryKey for Client<types::String, O> {}

impl IntoSecondaryKey for String {}

impl<'a> IntoSecondaryKey for &'a String {}

impl<'a> IntoSecondaryKey for &'a str {}

impl<O> IntoSecondaryKey for Client<types::Number, O> {}

impl IntoSecondaryKey for f32 {}

impl IntoSecondaryKey for i32 {}

impl IntoSecondaryKey for u32 {}

impl IntoSecondaryKey for f64 {}

impl IntoSecondaryKey for i64 {}

impl IntoSecondaryKey for u64 {}

impl<O> IntoSecondaryKey for Client<types::Bool, O> {}

impl IntoSecondaryKey for bool {}
