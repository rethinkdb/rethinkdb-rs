extern crate reql;
extern crate futures;
extern crate serde_json;

use reql::{r, Command};
use futures::stream::Stream;
use serde_json::Value;

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
