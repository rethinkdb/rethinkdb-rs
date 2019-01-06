pub(crate) mod opt;

use std::net::SocketAddr;

use crate::{net::connection::Connection, r, Result};
use futures::prelude::*;
use romio::TcpStream;

pub use self::opt::*;

impl r {
    /// Create a new connection to the database server
    ///
    /// You can change the default connection options using [Opts].
    ///
    /// ## Example
    ///
    /// Open a connection using the default host and port,
    /// specifying the default database.
    ///
    /// ```rust
    /// # use reql::{r, cmd::connect::Opts};
    /// #
    /// r.connect(Opts::builder().db("marvel").build())
    /// # ;
    /// ```
    ///
    /// The connection is created asynchronously, so you will have to `await`
    /// the result to get an actual connection.
    ///
    /// ## Example
    ///
    /// Open a new connection to the database.
    ///
    /// ```rust
    /// # use reql::{r, cmd::connect::Opts};
    /// #
    /// let opts = Opts::builder()
    ///     .host([127, 0, 0, 1])
    ///     .port(28015)
    ///     .db("marvel")
    ///     .build();
    /// r.connect(opts)
    /// # ;
    /// ```
    ///
    /// ## Example
    ///
    /// Open a new connection to the database, specifying a
    /// user/password combination for authentication.
    ///
    /// ```rust
    /// # use reql::{r, cmd::connect::Opts};
    /// #
    /// let opts = Opts::builder()
    ///     .host([127, 0, 0, 1])
    ///     .port(28015)
    ///     .db("marvel")
    ///     .user("herofinder")
    ///     .password("metropolis")
    ///     .build();
    /// r.connect(opts)
    /// # ;
    /// ```
    ///
    /// ## Related commands
    ///
    /// - [use_db]
    ///
    /// [Opts]: cmd/connect/struct.Opts.html
    /// [use_db]: cmd/connect/struct.Connection.html#method.use_db
    pub fn connect<'a, O: 'a>(self, opts: O) -> impl Future<Output = Result<Connection>> + 'a
    where
        O: Into<Opts<'a>>,
    {
        async move {
            let opts = opts.into();
            let stream = {
                let addr = SocketAddr::new(opts.host, opts.port);
                await!(TcpStream::connect(&addr))?
            };
            await!(Connection::new(opts.db, stream).hand_shake(opts))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::r;
    use futures::executor::block_on;

    #[test]
    fn driver_can_connect() -> crate::Result<()> {
        block_on(
            async {
                await!(r.connect(()))?;
                Ok(())
            },
        )
    }
}
