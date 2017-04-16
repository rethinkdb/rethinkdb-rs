extern crate tokio_core;
extern crate futures;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate reql;

use reql::{Client, Run};
use tokio_core::reactor::Core;
use futures::stream::Stream;

#[derive(Deserialize, Debug)]
struct ClusterConfig {
    id: String,
    heartbeat_timeout_secs: u32,
}

fn main() {
    // Create a new ReQL client with the logger
    let r = Client::new();

    // Create an even loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let conn = r.connect(args!(core.handle(), {servers: ["localhost"]})).unwrap();
    
    // Run the query
    let cfg = r.db("rethinkdb").table("cluster_config").run::<ClusterConfig>(conn).unwrap();
    for cfg in cfg.wait() {
        println!("{:?}", cfg);
    }
}
