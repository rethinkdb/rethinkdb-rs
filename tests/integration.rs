extern crate reql;

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
    let res = r.expr(200).run::<Value>().unwrap();
    for _ in res.wait() { }

    // Create our database if necessary
    let res = r.db_create("blog").run::<Value>().unwrap();
    for _ in res.wait() { }

    // Drop table if nessary
    let res = r.table_drop("users").run::<Value>().unwrap();
    for _ in res.wait() { }

    // Create our table if necessary
    let res = r.table_create("users").run::<Value>().unwrap();
    for _ in res.wait() { }

    /*
    // Insert a user into the table
    let user = r.object(vec![("name", "John Doe")]);
    let res: Response<Value> = r.table("users").insert(user).run().unwrap();
    let res = res.and_then(|v| {
        println!("Result: {:?}", v);
        Ok(())
    });
    for _ in res.wait() { }
    */
}
