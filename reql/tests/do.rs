use futures::TryStreamExt;
use reql::{func, r};
use serde_json::Value;

#[tokio::test]
async fn do_query() -> reql::Result<()> {
    env_logger::init();
    let conn = r.connect(()).await?;
    let mut query = r.do_(r.args(([10, 20], func!(|x, y| x + y)))).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    Ok(())
}
