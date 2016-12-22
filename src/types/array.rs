use types::data;

pub trait IntoArray {
    fn into_array(self) -> data::Array;
}
