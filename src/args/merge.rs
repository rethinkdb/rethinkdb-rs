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
                    let func = func!(self, var!(idx));
                    vec![func]
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
                    let func = func!(self.1, var!(idx), var!(idx));
                    vec![arg, func]
                }
            }
    };
}

merge!{ ObjectSelection for Object }
merge!{ Object for Object }
merge!{ Stream for Stream }
merge!{ StreamSelection for Stream }
merge!{ Table for Stream }
merge!{ Array for Array }
merge!{ ArraySelection for Array }
