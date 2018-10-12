#![feature(proc_macro_hygiene)]

extern crate futures;
extern crate reql;
extern crate reql_macros;
extern crate reql_types;

use reql::{Config, Client, Document, Run};
use reql_types::{Change, ServerStatus};
use reql_macros::args;
use futures::Stream;

fn main() -> reql::Result<()> {
    // Create a new ReQL client
    let r = Client::new();

    // Create a connection pool
    let conn = r.connect(Config::default())?;

    // Run the query
    let stati = r.db("rethinkdb")
        .table("server_status")
        .changes()
        .with_args(args!({include_initial: true}))
        .run::<Change<ServerStatus, ServerStatus>>(conn)?;

    // Process results
    for res in stati.wait() {
        match res {
            Ok(Some(Document::Expected(change))) => {
                println!("{:?}", change);
            }
            res => {
                println!("unexpected response from server: {:?}", res);
            }
        }
    }

    Ok(())
}
