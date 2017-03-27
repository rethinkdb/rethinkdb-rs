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
#[macro_use] extern crate reql;

use reql::Client;

fn main() {
    // Create a new ReQL client
    let r = Client::new();
    // Create a connection pool to your servers
    let conn = r.connect(args!({servers: ["localhost"], db: "blog"})).unwrap();
    // Run your ReQL commands
    let heroes = r.table("posts").get_all(args!("review", {index: "category"})).run(&conn).unwrap();
}
```

## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
