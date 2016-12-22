use types::data;

pub trait IntoFunction {
    fn into_function(self) -> data::Function;
}
