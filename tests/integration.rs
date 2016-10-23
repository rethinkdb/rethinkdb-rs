extern crate reql;
extern crate rayon;
#[macro_use] extern crate slog;

use reql::r;
use reql::session::Client;
use rayon::prelude::*;

#[test]
fn connection_pool_works() {
    let logger = Client::logger().read();
    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db("blog")
        .connect()
        .unwrap();

    // Create our database if necessary
    r.db_create("blog").run().unwrap();

    // Create our table if necessary
    r.table_create("users").run().unwrap();

    // Delete data if nessary
    r.table("users").delete().run().unwrap();

    // Insert 100 users into the table
    (0..100u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            let res = r.table("users").insert(user).run();
            debug!(logger, "{:?}", res);
        });
}
