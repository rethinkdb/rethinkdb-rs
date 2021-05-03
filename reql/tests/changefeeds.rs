use futures::stream::select_all;
use futures::TryStreamExt;
use reql::{r, Driver, Error};
use serde_json::Value;

#[test]
fn changefeeds_should_be_dedicated_to_a_connection() {
    match futures::executor::block_on(changefeeds()).unwrap_err() {
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
