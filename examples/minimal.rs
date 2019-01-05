#![feature(async_await, await_macro, futures_api)]

use reql::r;

fn main() -> reql::Result<()> {
    futures::executor::block_on(
        async {
            let conn = await!(r.connect(()))?;
            let resp: String = await!(r.expr("hello world").run(&conn))?;
            println!("{}", resp);
            Ok(())
        },
    )
}
