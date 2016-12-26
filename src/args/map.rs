use ::{Client, Term};
use types;
use commands::Arg;
use serde_json::value::ToJson;

pub trait IntoRootMapArg<I: types::DataType, O: types::DataType> {
    fn into_map_arg(self, idx: &mut u32) -> Vec<Term>;
}

pub trait IntoMapArg<I: types::DataType, O: types::DataType> {
    fn into_map_arg(self, idx: &mut u32) -> Vec<Term>;
}

macro_rules! map {
    ($arg:ident for $typ:ident) => {
        impl<C, T, O, F> IntoRootMapArg<types::$arg, types::$typ> for (Client<types::$arg, C>, F)
            where T: types::DataType,
                  O: ToJson + Clone,
                  C: ToJson + Clone,
                  F: Fn(Arg) -> Client<T, O>
            {
                fn into_map_arg(self, idx: &mut u32) -> Vec<Term> {
                    let arg: Term = self.0.into();
                    let res = self.1(var!(idx));
                    let func = func!(res.into(), idx, 1);
                    vec![arg, func]
                }
            }

        impl<F, T, O> IntoMapArg<types::$arg, types::$typ> for F
            where T: types::DataType,
                  O: ToJson + Clone,
                  F: Fn(Arg) -> Client<T, O>,
            {
                fn into_map_arg(self, idx: &mut u32) -> Vec<Term> {
                    let res = self(var!(idx));
                    let term = func!(res.into(), idx, 1);
                    vec![term]
                }
            }

        impl<F, C, T, O> IntoMapArg<types::$arg, types::$typ> for (Client<types::$arg, C>, F)
            where T: types::DataType,
                  O: ToJson + Clone,
                  C: ToJson + Clone,
                  F: Fn(Arg, Arg) -> Client<T, O>
            {
                fn into_map_arg(self, idx: &mut u32) -> Vec<Term> {
                    let arg: Term = self.0.into();
                    let res = self.1(var!(idx), var!(idx));
                    let func = func!(res.into(), idx, 2);
                    vec![arg, func]
                }
            }
    };
}

map!{ Array for Array }
map!{ Stream for Stream }
map!{ Table for Stream }
map!{ StreamSelection for Stream }
