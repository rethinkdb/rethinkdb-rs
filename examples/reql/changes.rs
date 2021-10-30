use futures::TryStreamExt;
use reql::r;
use reql::types::Change;
use serde_json::Value;

// We are using `tokio` here as an example but you can use this crate
// with any runtime
#[tokio::main]
async fn main() -> reql::Result<()> {
    // Initialise the logger if you need to debug this crate
    tracing_subscriber::fmt::init();

    // Create a RethinkDB connection out of the stream
    // See the API docs for more options you can configure
    let conn = r.connect(()).await?;

    // Create the query you want to run
    // The query returns a `Stream` of responses from RethinkDB
    let mut query = r.db("rethinkdb").table("jobs").changes(()).run(&conn);

    // Execute the query and handle the result
    while let Some(change) = query.try_next().await? {
        handle(change)?;
    }

    Ok(())
}

// We are just going to print the JSON response for this example
fn handle(change: Change<Value, Value>) -> reql::Result<()> {
    println!("{}", serde_json::to_string(&change)?);
    Ok(())
}
