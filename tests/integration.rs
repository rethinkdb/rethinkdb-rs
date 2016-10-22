extern crate reql;
extern crate rayon;

use reql::r;
use rayon::prelude::*;

#[test]
fn connection_pool_works() {
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .connect()
        .unwrap();

    (0..1_000_000u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            let _ = r.db("blog").table("users").insert(
                r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build()
                ).run().unwrap();
        });
}
