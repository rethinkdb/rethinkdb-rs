use types::data;

pub trait IntoObjectSelection {
    fn into_object_selection(self) -> data::ObjectSelection;
}
