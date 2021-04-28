use futures::stream::select_all;
use futures::StreamExt;
use reql::r;

#[tokio::test]
async fn expr() -> reql::Result<()> {
    env_logger::init();

    let conn = r.connect(()).await?;

    let mut streams = Vec::new();
    let num = 1_024u32;
    for i in 0..num {
        streams.push(r.expr(format!("message {}", i)).run::<_, String>(&conn));
    }

    let list = select_all(streams);
    assert_eq!(num, list.fold(0u32, |acc, _| async move { acc + 1 }).await);

    Ok(())
}
