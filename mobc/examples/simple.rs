use futures_lite::stream::StreamExt;
use mobc::Pool;
use mobc_reql::ReqlConnectionManager;
use reql::cmd::connect::Options;
use reql::r;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let manager = ReqlConnectionManager::new(Options::new());
    let pool = Pool::builder().max_open(20).build(manager);
    const MAX: usize = 5000;

    let now = Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<usize>(16);
    for i in 0..MAX {
        let pool = pool.clone();
        let tx_c = tx.clone();
        tokio::spawn(async move {
            let conn = pool.get().await.unwrap();
            let sum = r.expr(1) + r.expr(2);
            let value: i32 = sum.run(&*conn).try_next().await.unwrap().unwrap();
            assert_eq!(value, 3);
            tx_c.send(i).await.unwrap();
        });
    }
    for _ in 0..MAX {
        rx.recv().await.unwrap();
    }

    println!("cost: {:?}", now.elapsed());
}
