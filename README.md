# RethinkDB Driver

This is a [RethinkDB] driver written in [Rust].

[RethinkDB]: https://www.rethinkdb.com
[Rust]: https://www.rust-lang.org

[![Build Status](https://travis-ci.org/rust-rethinkdb/reql.svg?branch=master)](https://travis-ci.org/rust-rethinkdb/reql) [![Latest Version](https://img.shields.io/crates/v/reql.svg)](https://crates.io/crates/reql) [![Docs](https://docs.rs/reql/badge.svg)](https://docs.rs/reql)

## Example

```rust
extern crate futures;
extern crate tokio_core;
extern crate reql;
extern crate reql_types;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use reql::{Client, Run, Document};
use reql_types::ServerStatus;

fn main() {
    // Create a new ReQL client
    let r = Client::new();

    // Create an even loop
    let core = Core::new().unwrap();

    // Create a connection pool
    let conn = r.connect(&core.handle()).unwrap();

    // Run the query
    let query = r.db("rethinkdb").table("server_status").run::<ServerStatus>(conn).unwrap();

    // Process the results
    let stati = query.and_then(|status| {
        match status {
            // The server returned the response we were expecting
            Some(Document::Expected(change)) => {
                println!("{:?}", change);
            }
            // We got a response alright, but it wasn't the one were expecting
            // plus it's not an error either, otherwise it would have been
            // returned as such (This simply means that the response we got
            // couldn't be serialised into the type we were expecting)
            Some(Document::Unexpected(change)) => {
                println!("unexpected response from server: {:?}", change);
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
    })
    ;

    // Wait for all the results to be processed
    for _ in stati.wait() { }
}
```

Check out the [blocking example] to see this same example implemented using a for loop instead.

[blocking example]: https://github.com/rust-rethinkdb/reql/blob/master/examples/blocking.rs

## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
