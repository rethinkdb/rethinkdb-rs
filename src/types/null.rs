use types::data;

pub trait IntoNull {
    fn into_null(self) -> data::Null;
}
