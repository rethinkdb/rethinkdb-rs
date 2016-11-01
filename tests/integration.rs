extern crate reql;
extern crate rayon;

use reql::r;
use reql::prelude::*;
use rayon::prelude::*;

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
    for _ in res.wait() {}

    // Create our database if necessary
    let res: Response<Value> = r.db_create("blog").run().unwrap();
    for _ in res.wait() {}

    // Drop table if nessary
    let res: Response<Value> = r.table_drop("users").run().unwrap();
    for _ in res.wait() {}

    // Create our table if necessary
    let res: Response<Value> = r.table_create("users").run().unwrap();
    for _ in res.wait() {}

    // Insert 1 user(s) into the table
    (0..1u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            let res: Response<Value> = r.table("users").insert(user).run().unwrap();
            let res = res.for_each(|v| {
                println!("Result: {:?}", v);
                Ok(())
            });
            for _ in res.wait() {}
        });
}
