use types::data;

pub trait IntoObject {
    fn into_object(self) -> data::Object;
}
