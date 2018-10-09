extern crate reql;
extern crate reql_types;
extern crate futures;

#[macro_use]
extern crate serde_derive;

use reql::{Config, Client, Run};
use reql_types::Change;
use futures::Stream;
use std::net::IpAddr;
use std::net::SocketAddr;

// Create an easy structure that can be input in the rethinkdb webinterface
// with the following command in data explorer:
// r.db("test").table("testdata").insert({"data":"hello world!"})
#[derive(Serialize, Deserialize, Debug)]
struct TestChange {
    id: String,
    data: String,
}


// This example requires a rethinkdb with a db called "test" with a table called "testdata"
// It will printout the entire "Change" struct when a new entry is inserted/changed/deleted in
// the table
fn main() -> reql::Result<()> {
    // Create a new ReQL client
    let r = Client::new();

    // Set up connection parameters
    let mut conf = Config::default();

    // lets just recreate the default params
    let addr = "127.0.0.1".parse::<IpAddr>().unwrap();
    let socket = SocketAddr::new(addr, 28015);
    conf.db = "test";
    conf.servers = vec!(socket);
    conf.user = "admin";
    conf.password = "";

    // Create a connection pool
    let conn = r.connect(conf).unwrap();

    // Run the query on "testdata" table
    let query = r.db("test")
        .table("testdata")
        .changes()
        .run::<Change<TestChange, TestChange>>(conn)?;

    let mut changes = query.wait();

    // Process each new response from the database
    loop {
       let change = changes.next();
       match change {

            // The server responded with something
            Some(Ok(Some(data))) => {
                println!("{:?}", data);
            },
            Some(Err(e)) => {
                println!("Error {}", e);
            },
            _ => {
                println!("Something else happened");
            },
       };

    }
}
