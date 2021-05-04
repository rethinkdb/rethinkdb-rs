use futures::{StreamExt, TryStreamExt};
use futures_timer::Delay;
use log::error;
use reql::r;
use serde_json::Value;
use std::time::Duration;

// We are using `tokio` here as an example but you can use this crate
// with any runtime
#[tokio::main]
async fn main() -> reql::Result<()> {
    // Initialise the logger if you need to debug this crate
    env_logger::init();

    // Connect to create a RethinkDB session
    let session = r.connect(()).await?;

    // Manually get a connection from the session
    // Usually, this is not necessary (see other examples) but in this
    // case we need a handle to the connection so we can call close on
    // it later
    let mut connection = session.connection()?;

    // Clone the connection to get an instance to use for our feed below
    let conn = connection.clone();

    // Spawn the changefeed to run it in the background
    let feed_handle = tokio::spawn(async {
        // Create the query you want to run
        // The query returns a `Stream` of responses from RethinkDB
        let mut query = r.db("rethinkdb").table("jobs").changes(()).run(conn);

        // Execute the query and handle the result
        while let Some(change) = query.next().await {
            match change {
                // We are going to continue printing jobs until the feed is closed
                Ok(change) => {
                    if let Err(msg) = print_json(change) {
                        error!("failed to parse response; error: {}", msg);
                    }
                }
                Err(msg) => error!("feed returned an error: {}", msg),
            }
        }
    });

    // Delay a bit to let the feed run before closing it
    Delay::new(Duration::from_secs(2)).await;

    // and then close the changefeed
    connection.close(()).await?;

    // Wait for the feed to make sure it has finished running
    // This shouldn't block because we have closed the feed above
    let _ = feed_handle.await;

    // We can now use the same connection to run more queries
    // We wouldn't be able to do this otherwise since this driver
    // returns an error if you try to run more queries on a
    // connection that is running a changefeed
    let mut query = r.db("rethinkdb").table("server_status").run(connection);

    // Execute the query and print the result
    if let Some(server_status) = query.try_next().await? {
        print_json(server_status)?;
    }

    Ok(())
}

// We are just going to print the JSON response for this example
fn print_json(json: Value) -> reql::Result<()> {
    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
