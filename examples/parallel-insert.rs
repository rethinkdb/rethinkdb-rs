extern crate reql;
extern crate rayon;

use reql::r;
use rayon::prelude::*;

fn main() {
    let db = "blog";
    let table = "users";

    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db(db)
        .connect()
        .unwrap();

    // Create our database if necessary
    r.db_create(db).run().unwrap();

    // Drop table if nessary
    r.table_drop(table).run().unwrap();

    // Create our table
    r.table_create(table).run().unwrap();

    // Insert 200 user(s) into the table
    (0..200u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            r.table(table).insert(user).run().unwrap();
        });
}
