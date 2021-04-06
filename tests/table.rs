use async_std::net::TcpStream;
use futures::TryStreamExt;
use reql::{r, DEFAULT_ADDR};
use serde_json::Value;

#[async_std::test]
async fn table() -> reql::Result<()> {
    let stream = TcpStream::connect(DEFAULT_ADDR).await?;
    let conn = r.connection(stream).await?;
    let mut query = r.db("rethinkdb").table("users").run(&conn);
    let _: Option<Value> = query.try_next().await?;
    Ok(())
}
