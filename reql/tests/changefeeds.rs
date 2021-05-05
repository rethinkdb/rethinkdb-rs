use futures::stream::{select_all, TryStreamExt};
use reql::{r, Driver, Error};
use serde_json::Value;

#[tokio::test]
async fn changefeeds_should_use_dedicated_connections() {
    env_logger::init();

    match changefeeds().await.unwrap_err() {
        Error::Driver(Driver::ConnectionLocked) => {}
        error => panic!("{:?}", error),
    }
}

async fn changefeeds() -> reql::Result<()> {
    let conn = r.connect(()).await?;

    let _ = r
        .table_create("foo")
        .run::<_, Value>(&conn)
        .try_next()
        .await;
    let foo = r.table("foo").changes(()).run::<_, Value>(&conn);

    let _ = r
        .table_create("bar")
        .run::<_, Value>(&conn)
        .try_next()
        .await;
    let bar = r.table("bar").changes(()).run::<_, Value>(&conn);

    let mut list = select_all(vec![foo, bar]);

    while let Some(_) = list.try_next().await? {}

    Ok(())
}
