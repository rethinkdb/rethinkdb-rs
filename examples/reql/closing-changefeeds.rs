use futures::TryStreamExt;
use reql::r;
use serde_json::Value;

// We are using `tokio` here as an example but you can use this crate
// with any runtime
#[tokio::main]
async fn main() -> reql::Result<()> {
    // Initialise the logger if you need to debug this crate
    tracing_subscriber::fmt::init();

    // Connect to create a RethinkDB session
    let session = r.connect(()).await?;

    // Manually get a connection from the session
    // Usually, this is not necessary (see other examples) but in this
    // case we need a handle to the connection so we can call close on
    // it later
    let mut connection = session.connection()?;

    // Clone the connection to get an instance to use for our feed below
    let conn = connection.clone();

    // Create the query you want to run
    // The query returns a `Stream` of responses from RethinkDB
    let mut query = r.db("rethinkdb").table("jobs").changes(()).run(conn);

    // Execute the query and handle the result
    while let Some(change) = query.try_next().await? {
        // We are just going to print the first result
        print_json(change)?;
        // and then close the changefeed
        connection.close(()).await?;
        break;
    }

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
