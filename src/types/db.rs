use types::data;

pub trait IntoDb {
    fn into_db(self) -> data::Db;
}
