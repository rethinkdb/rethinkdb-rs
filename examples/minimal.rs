#![feature(async_await, await_macro, futures_api)]

use reql::{r, Response};

fn main() -> reql::Result<()> {
    env_logger::init();
    futures::executor::block_on(
        async {
            let conn = await!(r.connect(()))?;
            let resp: Response<String> = await!(r.expr("hello world").run(&conn))?;
            log::info!("result => {}", resp.first().unwrap());
            Ok(())
        },
    )
}
