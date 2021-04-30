use futures_lite::stream::StreamExt;
use mobc::{async_trait, Manager};
use reql::cmd::connect::Options;
use reql::{r, Client, Connection, Error, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReqlConnectionManager {
    opts: Options<'static>,
}

impl ReqlConnectionManager {
    pub const fn new(opts: Options<'static>) -> Self {
        Self { opts }
    }
}

#[async_trait]
impl Manager for ReqlConnectionManager {
    type Connection = Connection<'static>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection> {
        let conn = r.connect(self.opts).await?;
        Ok(conn.into_owned())
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection> {
        let msg = 200;
        if let Some(res) = r.expr(msg).run(&conn).try_next().await? {
            verify(res, msg)?;
        }
        Ok(conn)
    }
}

fn verify(res: u32, msg: u32) -> Result<()> {
    if res != msg {
        return Err(Client::ConnectionBroken.into());
    }
    Ok(())
}
