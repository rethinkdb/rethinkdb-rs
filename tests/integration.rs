extern crate reql;
extern crate rayon;

use reql::r;
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
    r.expr(200).run().unwrap();

    // Create our database if necessary
    r.db_create("blog").run().unwrap();

    // Drop table if nessary
    r.table_drop("users").run().unwrap();

    // Create our table if necessary
    r.table_create("users").run().unwrap();

    // Insert 1 user(s) into the table
    (0..1u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            r.table("users").insert(user).run().unwrap();
        });
}
