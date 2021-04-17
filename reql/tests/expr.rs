use async_std::net::TcpStream;
use futures::stream::select_all;
use futures::StreamExt;
use reql::{r, DEFAULT_ADDR};

#[async_std::test]
async fn expr() -> reql::Result<()> {
    env_logger::init();

    let stream = TcpStream::connect(DEFAULT_ADDR).await?;
    let conn = r.connection(stream).await?;

    let mut streams = Vec::new();
    let num = 1_024u32;
    for i in 0..num {
        streams.push(r.expr(format!("message {}", i)).run::<_, _, String>(&conn));
    }

    let list = select_all(streams);
    assert_eq!(num, list.fold(0u32, |acc, _| async move { acc + 1 }).await);

    Ok(())
}
