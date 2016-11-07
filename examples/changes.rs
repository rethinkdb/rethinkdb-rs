extern crate reql;

use reql::r;
use reql::prelude::*;

fn main() {
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db("blog")
        .connect()
        .unwrap();
    let request = r.table("users").changes().run::<Value>().unwrap();
    let response = request.and_then(|val| {
        println!("{:?}", val);
        Ok(())
    });
    for _ in response.wait() { }
}
