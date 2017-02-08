#[macro_use] extern crate reql;
#[macro_use] extern crate slog;
extern crate slog_term;

use slog::DrainExt;
use reql::commands::*;

fn main() {
    // Build an output drain
    let drain = slog_term::streamer().async().compact().build();
    // Setup the logger
    let logger = slog::Logger::root(drain.fuse(), o!("example" => "Logging"));
    // Create a new ReQL client with logger
    let r = Command::new().with_logger(logger);
    // Run command
    let _heroes = r.db("heroes").table(args!("marvel", {read_mode: "outdated"}));
}
