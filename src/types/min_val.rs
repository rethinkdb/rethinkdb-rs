use types::data;

pub trait IntoMinVal {
    fn into_min_val(self) -> data::MinVal;
}
