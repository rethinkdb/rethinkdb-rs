# RethinkDB Driver

This is a [RethinkDB] driver written in [Rust].

[RethinkDB]: https://www.rethinkdb.com
[Rust]: https://www.rust-lang.org

[![Build Status](https://travis-ci.org/rust-rethinkdb/reql.svg?branch=master)](https://travis-ci.org/rust-rethinkdb/reql) [![Build status](https://ci.appveyor.com/api/projects/status/cp8tmb9xxjw0kfgj?svg=true)](https://ci.appveyor.com/project/rushmorem/reql) [![Latest Version](https://img.shields.io/crates/v/reql.svg)](https://crates.io/crates/reql) [![Docs](https://docs.rs/reql/badge.svg)](https://docs.rs/reql/*/reql/struct.Client.html)

*Note:* At the moment, version `0.0.5` is the only usable version of this driver (version `0.0.6` hasn't been released yet). However, it only has a few commands implemented and the API has since changed. Version `0.0.6` will have all the ReQL commands implemented. It's going to be released sometime in April.

## Getting Started

Add this crate to your dependencies section:-

```toml
[dependencies]
reql = { git = "https://github.com/rust-rethinkdb/reql" }
```

Run ReQL commands:-

```rust
extern crate tokio_core;
extern crate futures;
extern crate reql;

use tokio_core::reactor::Core;
use futures::stream::Stream;
use reql::{Client, Run, ResponseValue};
use reql::structs::ServerStatus;

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
            Some(ResponseValue::Expected(change)) => {
                println!("{:?}", change);
            }
            // We got a response alright, but it wasn't the one were expecting
            // plus it's not an error either, otherwise it would have been
            // returned as such (This simply means that the response we got
            // couldn't be serialised into the type we were expecting)
            Some(ResponseValue::Unexpected(change)) => {
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

Checkout the [blocking example] to see this same example implemented using a for loop instead.

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
