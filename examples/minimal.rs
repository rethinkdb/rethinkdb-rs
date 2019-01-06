#![feature(async_await, await_macro, futures_api)]

use reql::r;

fn main() -> reql::Result<()> {
    futures::executor::block_on(
        async {
            let conn = await!(r.connect(()))?;
            let resp = await!(r.expr("hello world").run(&conn))?;
            let msg: &String = resp.first().unwrap();
            println!("server response => {:?}", msg);
            Ok(())
        },
    )
}
