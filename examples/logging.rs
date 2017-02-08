#[macro_use] extern crate reql;
#[macro_use] extern crate slog;
extern crate slog_term;

use slog::DrainExt;
use reql::Command;
use reql::commands::{Db, Table};

fn main() {
    // Build an output drain
    let drain = slog_term::streamer().async().compact().build();

    // Setup a logger
    let logger = slog::Logger::root(drain.fuse(), o!());

    // Create a new ReQL client with the logger
    let r = Command::new().with_logger(logger);

    // Run a command
    let _heroes = r.db("heroes").table(args!("marvel", {read_mode: "outdated"}));
}
