mod cmd;
mod err;

pub use cmd::*;
pub use err::*;

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

#[allow(non_camel_case_types)]
pub struct r;

#[derive(Debug, Clone)]
pub struct Command {
    query: ql2::Query,
}

#[derive(Debug)]
pub struct Connection {
    stream: tokio::net::TcpStream,
}
