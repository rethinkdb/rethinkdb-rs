use futures::executor;
use futures::stream::select_all;
use reql::r;

#[test]
fn concurrency() -> reql::Result<()> {
    let conn = executor::block_on(r.connect(()))?;

    let mut streams = Vec::new();

    let num = 10_000;
    for i in 0..num {
        streams.push(r.expr(format!("message {}", i)).run::<_, String>(&conn));
    }

    let list = select_all(streams);
    let mut iter = executor::block_on_stream(list);

    while let Some(msg) = iter.next() {
        msg.unwrap();
    }

    Ok(())
}
