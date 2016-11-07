extern crate reql;

use reql::r;
use reql::prelude::*;

#[test]
fn connection_pool_works() {
    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db("blog")
        .connect()
        .unwrap();

    // Try arbitrary expressions
    let res: Response<Value> = r.expr(200).run().unwrap();
    for _ in res.wait() { }

    // Create our database if necessary
    let res: Response<Value> = r.db_create("blog").run().unwrap();
    for _ in res.wait() { }

    // Drop table if nessary
    let res: Response<Value> = r.table_drop("users").run().unwrap();
    for _ in res.wait() { }

    // Create our table if necessary
    let res: Response<Value> = r.table_create("users").run().unwrap();
    for _ in res.wait() { }

    // Insert a user into the table
    let user = r.object()
        .insert("name", "John Doe")
        .build();
    let res: Response<Value> = r.table("users").insert(user).run().unwrap();
    let res = res.and_then(|v| {
        println!("Result: {:?}", v);
        Ok(())
    });
    for _ in res.wait() { }
}
