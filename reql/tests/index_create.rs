use futures::TryStreamExt;
use reql::macros::func;
use reql::r;
use serde_json::Value;

#[tokio::test]
async fn index_create() -> reql::Result<()> {
    env_logger::init();

    let conn = r.connect(()).await?;

    let _ = r
        .table_create("comments")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_drop("author_name")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let mut query = r
        .table("comments")
        .index_create((
            "author_name",
            func!(|doc| { doc.get_field("author").bracket("name") }),
        ))
        .run(&conn);

    let user: Option<Value> = query.try_next().await?;
    assert!(user.is_some());

    Ok(())
}
