extern crate slog_term;
#[macro_use] extern crate slog;
extern crate tokio_core;
extern crate futures;

#[macro_use] extern crate reql;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use slog::DrainExt;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use serde_json::value::to_value;

use reql::{Client, Run, ResponseValue};
use reql::structs::WriteStatus;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    // Build an output drain
    let drain = slog_term::streamer().async().compact().build();

    // Setup a logger
    let logger = slog::Logger::root(drain.fuse(), o!());

    // Create a new ReQL client with the logger
    let r = Client::new().with_logger(logger);

    // Create an even loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let conn = r.connect(args!(core.handle(), {servers: ["localhost"]})).unwrap();

    // Create the table
    match r.db("test").table_create(args!("users", {replicas: 3})).run::<()>(conn).unwrap().wait().next().unwrap() {
        Ok(Some(ResponseValue::Expected(status))) => {
            println!("{:?}", status);
        }
        Ok(res) => {
            println!("unexpected response from server: {:?}", res);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    for i in 0..30000 {
        // Run the query
        let user = User {
            name: format!("User {}", i),
            age: i,
        };
        let user = to_value(user).unwrap();
        match r.db("test").table("users").insert(user).run::<WriteStatus>(conn).unwrap().wait().next().unwrap() {
            Ok(Some(ResponseValue::Expected(status))) => {
                println!("{:?}", status);
            }
            Ok(res) => {
                println!("unexpected response from server: {:?}", res);
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}
