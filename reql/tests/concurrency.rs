use futures::stream::select_all;
use futures::TryStreamExt;
use reql::r;

#[tokio::test]
async fn concurrency() -> reql::Result<()> {
    env_logger::init();

    let conn = r.connect(()).await?;

    let mut streams = Vec::new();
    let num = 10_000;
    for i in 0..num {
        streams.push(r.expr(format!("message {}", i)).run::<_, String>(&conn));
    }

    let mut list = select_all(streams);

    while let Some(msg) = list.try_next().await? {
        log::debug!("{}", msg);
    }

    Ok(())
}
