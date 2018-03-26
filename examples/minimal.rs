extern crate reql;
extern crate reql_types;
extern crate futures_await as futures;

use reql::{Config, Client, Document, Run};
use reql_types::ServerStatus;
use futures::StreamExt;

fn main() {
    // Create a new ReQL client
    let r = Client::new();

    // Create a connection pool
    let conn = r.connect(Config::default()).unwrap();

    // Run the query
    let query = r.db("rethinkdb")
        .table("server_status")
        .run::<ServerStatus>(conn)
        .unwrap();

    // Process the results
    let stati = query.and_then(|status| {
        match status {
            // The server returned the response we were expecting
            Some(Document::Expected(status)) => {
                println!("{:?}", status);
            }
            // We got a response alright, but it wasn't the one we were
            // expecting plus it's not an error either, otherwise it would
            // have been returned as such (This simply means that the response
            // we got couldn't be serialised into the type we were expecting)
            Some(Document::Unexpected(status)) => {
                println!("unexpected response from server: {:?}", status);
            }
            // This is impossible in this particular example since there
            // needs to be at least one server available to give this
            // response otherwise we would have run into an error for
            // failing to connect
            None => {
                println!("got no documents in the database");
            }
        }
        Ok(())
    })
    // Our query ran into an error
    .or_else(|error| {
        println!("{:?}", error);
        Err(())
    });

    // Wait for all the results to be processed
    let _ = futures::executor::block_on(stati.into_future());
}
