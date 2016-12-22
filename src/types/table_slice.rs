use types::data;

pub trait IntoTableSlice {
    fn into_table_slice(self) -> data::TableSlice;
}
