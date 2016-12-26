use {Client, Term};
use types;
use commands::Arg;
use serde_json::value::ToJson;

pub trait IntoMergeArg<I: types::DataType, O: types::DataType> {
    fn into_merge_arg(self, idx: &mut u32) -> Vec<Term>;
}

impl<O> IntoMergeArg<types::Object, types::Object> for Client<types::Stream, O>
    where O: ToJson + Clone
{
    fn into_merge_arg(self, _: &mut u32) -> Vec<Term> {
        vec![self.into()]
    }
}

macro_rules! merge {
    ($arg:ident for $typ:ident) => {
        impl<F, T, O> IntoMergeArg<types::$arg, types::$typ> for F
            where T: types::DataType,
                  O: ToJson + Clone,
                  F: Fn(Arg) -> Client<T, O>,
            {
                fn into_merge_arg(self, idx: &mut u32) -> Vec<Term> {
                    let res = self(var!(idx));
                    let term = func!(res.into(), idx, 1);
                    vec![term]
                }
            }

        impl<F, C, T, O> IntoMergeArg<types::$arg, types::$typ> for (Client<types::$arg, C>, F)
            where T: types::DataType,
                  O: ToJson + Clone,
                  C: ToJson + Clone,
                  F: Fn(Arg, Arg) -> Client<T, O>
            {
                fn into_merge_arg(self, idx: &mut u32) -> Vec<Term> {
                    let arg: Term = self.0.into();
                    let res = self.1(var!(idx), var!(idx));
                    let func = func!(res.into(), idx, 2);
                    vec![arg, func]
                }
            }
    };
}

merge!{ Array for Array }
merge!{ ArraySelection for Array }
merge!{ Stream for Stream }
merge!{ Table for Stream }
merge!{ StreamSelection for Stream }
