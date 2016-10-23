extern crate reql;
extern crate rayon;

use reql::r;
use rayon::prelude::*;

#[test]
fn connection_pool_works() {
    // Setup the connection
    r.connection()
        .set_db("blog")
        .connect()
        .unwrap();

    // Create our database
    r.db_create("blog").run().unwrap();

    // Create our table
    r.table_create("users").run().unwrap();

    // Insert 100 users into the table
    (0..100u32)
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
