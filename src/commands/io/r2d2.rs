use reql_io::r2d2;
use errors::Error;
use {ConnectionManager, InnerConnection, Result};

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = InnerConnection;
    type Error = Error;

    fn connect(&self) -> Result<InnerConnection> {
        //Connection::new(self.clone())
        unimplemented!();
    }

    //fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
    fn is_valid(&self, _: &mut InnerConnection) -> Result<()> {
        unimplemented!();
    }

    fn has_broken(&self, _conn: &mut InnerConnection) -> bool {
        //conn.broken()
        unimplemented!();
    }
}
