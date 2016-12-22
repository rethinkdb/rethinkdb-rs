use types::data;

pub trait IntoTime {
    fn into_time(self) -> data::Time;
}
