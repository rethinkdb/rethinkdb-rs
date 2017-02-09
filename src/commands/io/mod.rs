mod connect;
mod handshake;
mod r2d2;

use std::{io, error};

use {Config, Connection, Pool, ToArg, Result, Response};
use reql_io::tokio_core::reactor::Handle;

/// Create a new connection to the database server
pub trait Connect {
    fn connect(&self, cfg: Config, handle: &Handle) -> Result<Pool>;
}

/// Run the query
pub trait Run<T>
    where T: ToArg
{
    fn run<R>(&self, args: T) -> Response<R>;
}

fn io_error<T>(err: T) -> io::Error
    where T: error::Error + Send + Sync + 'static
{
    io::Error::new(io::ErrorKind::Other, err)
}
