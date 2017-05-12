extern crate futures;
extern crate reql;
extern crate reql_types;
extern crate tokio_core;

use futures::stream::Stream;
use reql::{Client, Document, Run};
use reql_types::ServerStatus;
use tokio_core::reactor::Core;

fn main()
{
    // Create a new ReQL client
    let r = Client::new();

    // Create an even loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let conn = r.connect(&core.handle()).unwrap();

    // Run the query
    let stati = r.db("rethinkdb")
        .table("server_status")
        .run::<ServerStatus>(conn)
        .unwrap();

    // Process the results
    for res in stati.wait() {
        match res {
            // The server returned the response we were expecting
            Ok(Some(Document::Expected(status))) => {
                println!("{:?}", status);
            }
            // We got a response alright, but it wasn't the one we were
            // expecting plus it's not an error either, otherwise it would
            // have been returned as such (This simply means that the response
            // we got couldn't be serialised into the type we were expecting)
            Ok(Some(Document::Unexpected(status))) => {
                println!("unexpected response from server: {:?}", status);
            }
            // This is impossible in this particular example since there
            // needs to be at least one server available to give this
            // response otherwise we would have run into an error for
            // failing to connect
            Ok(None) => {
                println!("got no documents in the database");
            }
            // Our query ran into an error
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}
