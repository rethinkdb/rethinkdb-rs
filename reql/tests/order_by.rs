use futures::TryStreamExt;
use reql::r;
use serde_json::Value;

#[tokio::test]
async fn order_by() -> reql::Result<()> {
    env_logger::init();
    let conn = r.connect(()).await?;
    let mut query = r
        .db("rethinkdb")
        .table("server_status")
        .order_by(r.args(("name", r.index(r.desc("id")))))
        .run(&conn);
    let user: Option<Value> = query.try_next().await?;
    assert!(user.is_some());
    Ok(())
}
