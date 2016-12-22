use types::data;

pub trait IntoMaxVal {
    fn into_max_val(self) -> data::MaxVal;
}
