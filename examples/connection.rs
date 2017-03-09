extern crate slog_term;
extern crate reql_io;
#[macro_use] extern crate slog;
#[macro_use] extern crate reql;

use slog::DrainExt;
use reql::Client;
use reql_io::tokio_core::reactor::Core;

fn main() {
    // Build an output drain
    let drain = slog_term::streamer().async().compact().build();

    // Setup a logger
    let logger = slog::Logger::root(drain.fuse(), o!());

    // Create a new ReQL client with the logger
    let r = Client::new().with_logger(logger);

    // Create the event loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let _pool = r.connect(args!()).unwrap();
}
