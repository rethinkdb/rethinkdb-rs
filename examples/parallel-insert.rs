extern crate reql;
extern crate rayon;

use reql::r;
use rayon::prelude::*;

fn main() {
    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db("blog")
        .connect()
        .unwrap();

    /*
    // Create our database if necessary
    r.db_create(db).run().unwrap();

    // Drop table if nessary
    r.table_drop("users").run().unwrap();

    // Create our table if necessary
    r.table_create("users").run().unwrap();
    */

    // Insert 20000 user(s) into the table
    (0..20000u32)
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
