use {Client, Term};
use types;
use commands::Arg;
use serde_json::value::ToJson;

pub trait IntoFilterArg<I: types::DataType, O: types::DataType> {
    fn into_filter_arg(self, idx: &mut u32) -> Vec<Term>;
}

macro_rules! filter {
    ($arg:ident for $typ:ident) => {
        impl<F, T, O> IntoFilterArg<types::$arg, types::$typ> for F
            where T: types::DataType,
                  O: ToJson + Clone,
                  F: FnOnce(Arg) -> Client<T, O>,
            {
                fn into_filter_arg(self, idx: &mut u32) -> Vec<Term> {
                    let func = func!(self, var!(idx));
                    vec![func]
                }
            }

        impl<F, C, T, O> IntoFilterArg<types::$arg, types::$typ> for (Client<types::$arg, C>, F)
            where T: types::DataType,
                  O: ToJson + Clone,
                  C: ToJson + Clone,
                  F: FnOnce(Arg, Arg) -> Client<T, O>
            {
                fn into_filter_arg(self, idx: &mut u32) -> Vec<Term> {
                    let arg: Term = self.0.into();
                    let func = func!(self.1, var!(idx), var!(idx));
                    vec![arg, func]
                }
            }
    };
}

filter!{ Stream for Stream }
filter!{ StreamSelection for Stream }
filter!{ Table for Stream }
filter!{ Array for Array }
filter!{ ArraySelection for Array }
