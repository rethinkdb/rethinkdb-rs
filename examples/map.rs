extern crate slog_term;
#[macro_use] extern crate slog;
extern crate tokio_core;
extern crate futures;
extern crate serde_json;

#[macro_use] extern crate reql;

use slog::DrainExt;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use serde_json::value::ToJson;

use reql::{Client, Run, ResponseValue};

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
    
    // Run the query
    let sequence1 = [100, 200, 300, 400].to_json().unwrap();
    let sequence2 = [10, 20, 30, 40].to_json().unwrap();
    let sequence3 = [1, 2, 3, 4].to_json().unwrap();
    let sum = r.map(args!(sequence1, sequence2, sequence3, |val1, val2, val3| {
        val1.add(val2).add(val3)
    })).run::<[i32; 4]>(conn).unwrap();

    // Process results
    for res in sum.wait() {
        match res {
            Ok(Ok(Some(ResponseValue::Expected(sum)))) => {
                println!("{:?}", sum);
            }
            Ok(Ok(res)) => {
                println!("unexpected response from DB: {:?}", res);
            }
            Ok(Err(error)) => {
                println!("{:?}", error);
            }
            Err(_) => {
                println!("an error occured while processing the stream");
            }
        }
    }
}
