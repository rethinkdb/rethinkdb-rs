use types::data;

pub trait IntoArraySelection {
    fn into_array_selection(self) -> data::ArraySelection;
}
