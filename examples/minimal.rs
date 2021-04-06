use async_std::net::TcpStream;
use futures::TryStreamExt;
use reql::{r, DEFAULT_ADDR};
use reql_types::ServerStatus;

// We are using `async_std` here as an example but you can use this crate
// with any `TcpStream` that implements `AsyncRead` and `AsyncWrite` from
// the futures crate. `async_std` just works out of the box because it does.
//
// *NB*: `&TcpStream` needs to implement those traits as well since we
// use unmutable references of the connection to take advantage of RethinkDB's
// connection pipelining abilities.
#[async_std::main]
async fn main() -> reql::Result<()> {
    // Initialise the logger if you need to debug this crate
    env_logger::init();

    // Configure and connect the `TcpStream` you want to use
    let stream = TcpStream::connect(DEFAULT_ADDR).await?;

    // Create a RethinkDB connection out of the stream
    // See the API docs for more options you can configure
    let conn = r.connection(stream).await?;

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
