mod with_args;

use {Config, Connection, Result, Response};
use reql_io::tokio_core::reactor::Handle;

/// Create a new connection to the database server
pub trait Connect {
    fn connect(&self, cfg: Config, handle: &Handle) -> Result<Connection>;
}

/// Run the query
pub trait Run {
    fn run<T>(&self, conn: &Connection) -> Response<T>;
}
