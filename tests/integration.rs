/*
#[macro_use]
extern crate reql;
extern crate serde_json;

use reql::{r, Run};
use serde_json::Value;

#[test]
fn connection_pool_works() {
    // Setup the connection
    r.connection()
        .servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .db("blog")
        .connect()
        .unwrap();

    // Try arbitrary expressions
    let res = r.expr(200).run::<Value>();
    res.into_iter().next();

    // Create our database if necessary
    let res = r.db_create("blog").run::<Value>();
    res.into_iter().next();

    // Drop table if necessary
    let res = r.table_drop("users").run::<Value>();
    res.into_iter().next();

    // Create our table if necessary
    let res = r.table_create("users").run::<Value>();
    res.into_iter().next();

    // Insert a user into the table
    let res = r.table("users").insert(obj!{ name: "John Doe" }).run::<Value>();
    res.into_iter().next();
}
*/
