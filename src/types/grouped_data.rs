use types::data;

pub trait IntoGroupedData {
    fn into_grouped_data(self) -> data::GroupedData;
}
