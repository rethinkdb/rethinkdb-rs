use futures::stream::{select_all, TryStreamExt};
use reql::r;

#[tokio::test]
async fn concurrency() -> reql::Result<()> {
    let conn = r.connect(()).await?;

    let mut streams = Vec::new();

    let num = 10_000;
    for i in 0..num {
        streams.push(r.expr(format!("message {}", i)).run::<_, String>(&conn));
    }

    let mut list = select_all(streams);

    while list.try_next().await?.is_some() {}

    Ok(())
}
