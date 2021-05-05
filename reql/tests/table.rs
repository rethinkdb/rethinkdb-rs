use futures::TryStreamExt;
use reql::r;
use serde_json::Value;

#[tokio::test]
async fn table() -> reql::Result<()> {
    env_logger::init();
    let conn = r.connect(()).await?;
    let mut query = r.db("rethinkdb").table("users").run(&conn);
    let user: Option<Value> = query.try_next().await?;
    assert!(user.is_some());
    Ok(())
}
