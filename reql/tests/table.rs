use futures::executor;
use reql::{r, Result};
use serde_json::Value;

#[test]
fn table() -> Result<()> {
    let conn = executor::block_on(r.connect(()))?;
    let query = r.db("rethinkdb").table("users").run(&conn);
    let _: Option<Result<Value>> = executor::block_on_stream(query).next();
    Ok(())
}
