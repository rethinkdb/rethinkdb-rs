use futures::executor::block_on;
use reql::r;

#[test]
fn queries_are_thread_safe() {
    env_logger::init();
    let conn = block_on(r.connect(())).unwrap();
    crossbeam::scope(|thread| {
        for i in 0..10 {
            let conn = &conn;
            thread.spawn(move |_| {
                let msg = format!("iteration {}", i);
                let resp = block_on(r.expr(&msg).run(conn)).unwrap();
                let first: &String = resp.first().unwrap();
                if &msg != first {
                    log::error!("{} != {}", msg, first);
                    std::process::exit(1);
                } else {
                    log::info!("result => {}", first);
                }
            });
        }
    })
    .unwrap();
}
