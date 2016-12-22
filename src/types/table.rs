use types::data;

pub trait IntoTable {
    fn into_table(self) -> data::Table;
}
