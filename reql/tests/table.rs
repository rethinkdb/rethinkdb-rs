use futures::TryStreamExt;
use reql::r;
use serde_json::Value;

#[tokio::test]
async fn table() -> reql::Result<()> {
    env_logger::init();
    let conn = r.connect(()).await?;
    let mut query = r.db("rethinkdb").table("users").run(&conn);
    let _: Option<Value> = query.try_next().await?;
    Ok(())
}
