extern crate futures_await as futures;
#[macro_use]
extern crate reql;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_term;

use futures::executor::block_on;
use futures::StreamExt;
use reql::{Client, Document, Run};
use slog::Drain;

fn main() {
    // Build an output drain
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(plain).build();

    // Setup a logger
    let logger = slog::Logger::root(drain.fuse(), o!());

    // Create a new ReQL client with the logger
    let r = Client::new().with_logger(logger);

    // Create a connection pool
    let conn = r.connect(args!({servers: ["localhost"]}))
        .unwrap();

    // Run the query
    let sequence1 = json!([100, 200, 300, 400]);
    let sequence2 = json!([10, 20, 30, 40]);
    let sequence3 = json!([1, 2, 3, 4]);

    let sum = r.map(args!(sequence1, sequence2, sequence3, |val1, val2, val3| {
        val1.add(val2).add(val3)
    }))
    .run::<[i32; 4]>(conn)
        .unwrap();

    // Process results
    match block_on(sum.into_future()).unwrap().0.unwrap() {
        Some(Document::Expected(sum)) => {
            println!("{:?}", sum);
        }
        res => {
            println!("unexpected response from DB: {:?}", res);
        }
    }
}
