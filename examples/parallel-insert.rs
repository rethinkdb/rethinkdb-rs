extern crate reql;
extern crate rayon;

use reql::r;
use reql::prelude::*;
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
    let res: Response<Value> = r.db_create(db).run().unwrap();
    let _ = res.wait();

    // Drop table if nessary
    let res: Response<Value> = r.table_drop(table).run().unwrap();
    let _ = res.wait();

    // Create our table
    let res: Response<Value> = r.table_create(table).run().unwrap();
    let _ = res.wait();

    // Insert 200 user(s) into the table
    (0..200u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            let res: Response<Value> = r.table(table).insert(user).run().unwrap();
            let _ = res.wait();
        });
}
