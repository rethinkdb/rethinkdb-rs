extern crate futures;
#[macro_use]
extern crate reql;
extern crate reql_types;
#[macro_use]
extern crate slog;
extern crate slog_term;

use reql::{Config, Client, Document, Run};
use reql_types::{Change, ServerStatus};
use futures::executor::block_on_stream;
use futures::StreamExt;
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
    let conn = r.connect(Config::default()).unwrap();

    // Run the query
    let stati = r.db("rethinkdb")
        .table("server_status")
        .changes()
        .with_args(args!({include_initial: true}))
        .run::<Change<ServerStatus, ServerStatus>>(conn)
        .unwrap();

    // Process results
    match block_on_stream(stati).unwrap().0.unwrap() {
        Some(Document::Expected(change)) => {
            println!("{:?}", change);
        }
        res => {
            println!("unexpected response from server: {:?}", res);
        }
    }
}
