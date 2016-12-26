pub trait IntoDb {
    fn into_db(self) -> ::types::Db;
}
