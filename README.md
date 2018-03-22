# RethinkDB Driver

This is a [RethinkDB] driver written in [Rust].

[![travis-badge][]][travis] [![cratesio-badge][]][cratesio] [![docsrs-badge][]][docsrs] [![rust-version-badge][]][rust-version]

## Example

```rust
extern crate futures;
extern crate reql;
extern crate reql_types;
extern crate tokio_core;

use futures::stream::Stream;
use reql::{Client, Document, Run};
use reql_types::ServerStatus;
use tokio_core::reactor::Core;

fn main() {
    // Create a new ReQL client
    let r = Client::new();

    // Create an even loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let conn = r.connect(&core.handle()).unwrap();

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
    for _ in stati.wait() {}
}
```

Check out the [blocking example] to see this same example implemented using a for loop instead.

## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[RethinkDB]: https://www.rethinkdb.com
[Rust]: https://www.rust-lang.org
[travis-badge]: https://travis-ci.org/rethinkdb-rs/reql.svg?branch=master
[travis]: https://travis-ci.org/rethinkdb-rs/reql
[cratesio-badge]: https://img.shields.io/crates/v/reql.svg
[cratesio]: https://crates.io/crates/reql
[docsrs-badge]: https://docs.rs/reql/badge.svg
[docsrs]: https://docs.rs/reql
[rust-version-badge]: https://img.shields.io/badge/rust-nightly%202018--03--01-blue.svg
[rust-version]: .travis.yml#L7
[blocking example]: https://github.com/rethinkdb-rs/reql/blob/master/examples/blocking.rs
