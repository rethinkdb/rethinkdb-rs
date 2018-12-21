use crate::{Client, Result};
//use futures::prelude::*;
use romio::TcpStream;

#[derive(Debug)]
struct Session {
    id: u64,
    stream: TcpStream,
}

#[derive(Debug)]
pub struct Connection<'a> {
    client: Client<'a>,
    session: Session,
}

impl<'a> Connection<'a> {
    pub(crate) async fn new(client: Client<'a>, id: u64) -> Result<Connection<'a>> {
        let cfg = client.config();
        let stream = await!(TcpStream::connect(cfg.server()))?;
        let session = Session { id, stream };
        let conn = Connection { client, session };
        await!(conn.shake_hands())
    }

    async fn shake_hands(self) -> Result<Connection<'a>> {
        Ok(self)
    }
}
