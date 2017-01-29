mod handshake;

use {Result, Connection, AsConnection};
use commands::{Command, Args};

/// Create a new connection to the database server
pub trait Connect {
    type Connection: AsConnection;

    fn connect(&self, args: Args) -> Result<Self::Connection>;
}

impl Connect for Command {
    type Connection = Connection;

    fn connect(&self, args: Args) -> Result<Connection>
    {
        Ok(Connection { })
    }
}

impl AsConnection for Connection {
}
