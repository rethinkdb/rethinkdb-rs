use futures::TryStreamExt;
use reql::r;
use reql_types::ServerStatus;

// We are using `tokio` here as an example but you can use this crate
// with any runtime
#[tokio::main]
async fn main() -> reql::Result<()> {
    // Initialise the logger if you need to debug this crate
    env_logger::init();

    // Create a RethinkDB connection out of the stream
    // See the API docs for more options you can configure
    let conn = r.connect(()).await?;

    // Create the query you want to run
    // The query returns a `Stream` of responses from RethinkDB
    let mut query = r.db("rethinkdb").table("server_status").run(&conn);

    // Execute the query and handle the result
    if let Some(server_status) = query.try_next().await? {
        handle(&server_status)?;
    }
    Ok(())
}

// We are just going to print the JSON response for this example
fn handle(server_status: &ServerStatus) -> reql::Result<()> {
    println!("{}", serde_json::to_string_pretty(server_status)?);
    Ok(())
}
