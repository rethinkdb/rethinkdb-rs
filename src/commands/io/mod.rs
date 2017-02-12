mod connect;
mod run;

use std::{io, error};

use {Config, Connection, Pool, ToArg, Result, Response};
use reql_io::tokio_core::reactor::Handle;

/// Create a new connection to the database server
pub trait Connect {
    fn connect(&self, config: Config, handle: &Handle) -> Result<Pool>;
}

/// Run the query
pub trait Run<T>
    where T: ToArg
{
    fn run<R>(&self, args: T) -> Response<R>;
}

fn io_error<T>(err: T) -> io::Error
    where T: Into<Box<error::Error + Send + Sync>>
{
    io::Error::new(io::ErrorKind::Other, err)
}
