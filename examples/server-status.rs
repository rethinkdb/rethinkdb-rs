extern crate futures_await as futures;
#[macro_use]
extern crate reql;
extern crate reql_types;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate tokio;

use futures::stream::Stream;
use reql::{Client, Document, Run};
use reql_types::{Change, ServerStatus};
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
    let stati = r.db("rethinkdb")
        .table("server_status")
        .changes()
        .with_args(args!({include_initial: true}))
        .run::<Change<ServerStatus, ServerStatus>>(conn)
        .unwrap();

    // Process results
    for res in stati.wait() {
        match res {
            Ok(Some(Document::Expected(change))) => {
                println!("{:?}", change);
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
