use ::{Client, Term};
use types::data;
use commands::Arg;
use serde_json::value::ToJson;

pub trait IntoMapArg<T: data::DataType> {
    fn into_map_arg(self, idx: &mut u32) -> Vec<Term>;
}

impl<D, F, T, O> IntoMapArg<D> for F
    where F: Fn(Arg) -> Client<T, O>,
          T: data::DataType,
          D: data::DataType,
          O: ToJson + Clone,
{
    fn into_map_arg(self, idx: &mut u32) -> Vec<Term> {
        let res = self(var!(idx));
        let term = func!(res.into(), idx, 1);
        vec![term]
    }
}

impl<D, F, CT, CO, T, O> IntoMapArg<D> for (Client<CT, CO>, F)
    where CT: data::DataType,
          CO: ToJson + Clone,
          D: data::DataType,
          T: data::DataType,
          O: ToJson + Clone,
          F: Fn(Arg, Arg) -> Client<T, O>
{
    fn into_map_arg(self, idx: &mut u32) -> Vec<Term> {
        let arg: Term = self.0.into();
        let res = self.1(var!(idx), var!(idx));
        let func = func!(res.into(), idx, 2);
        vec![arg, func]
    }
}
