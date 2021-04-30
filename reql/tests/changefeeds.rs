use futures::stream::select_all;
use futures::TryStreamExt;
use reql::{r, Client, Error};
use reql_types::Change;
use serde_json::Value;

#[tokio::test]
async fn changefeeds_should_be_dedicated_to_a_connection() {
    match changefeeds().await.unwrap_err() {
        Error::Client(Client::ConnectionLocked) => {}
        error => panic!("{:?}", error),
    }
}

async fn changefeeds() -> reql::Result<()> {
    env_logger::init();

    let conn = r.connect(()).await?;

    let _ = r
        .table_create("foo")
        .run::<_, Value>(&conn)
        .try_next()
        .await;
    let foo = r.table("foo").changes(()).run(&conn);

    let _ = r
        .table_create("bar")
        .run::<_, Value>(&conn)
        .try_next()
        .await;
    let bar = r.table("bar").changes(()).run(&conn);

    let mut list = select_all(vec![foo, bar]);

    while let Some(change) = list.try_next().await? {
        handle(change)?;
    }
    Ok(())
}

fn handle(change: Change<Value, Value>) -> reql::Result<()> {
    log::debug!("{}", serde_json::to_string(&change)?);
    Ok(())
}
