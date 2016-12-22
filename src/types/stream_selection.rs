use types::data;

pub trait IntoStreamSelection {
    fn into_stream_selection(self) -> data::StreamSelection;
}
