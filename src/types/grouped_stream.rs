use types::data;

pub trait IntoGroupedStream {
    fn into_grouped_stream(self) -> data::GroupedStream;
}
