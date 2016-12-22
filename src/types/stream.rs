use types::data;

pub trait IntoStream {
    fn into_stream(self) -> data::Stream;
}
