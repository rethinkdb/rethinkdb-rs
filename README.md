# RethinkDB Driver

This is a [RethinkDB] driver written in [Rust].

[RethinkDB]: https://www.rethinkdb.com
[Rust]: https://www.rust-lang.org

[![Build Status](https://travis-ci.org/rust-rethinkdb/reql.svg?branch=master)](https://travis-ci.org/rust-rethinkdb/reql) [![Build status](https://ci.appveyor.com/api/projects/status/cp8tmb9xxjw0kfgj?svg=true)](https://ci.appveyor.com/project/rushmorem/reql) [![Latest Version](https://img.shields.io/crates/v/reql.svg)](https://crates.io/crates/reql) [![Docs](https://docs.rs/reql/badge.svg)](https://docs.rs/reql)

*Note:* While this driver is already usable in the current state, the API is not yet stable and many commands are not yet implemented. I recommend you pin to specific versions if you have to code against it. Also kindly submit an issue or pull request if the command you want is missing.

## Getting Started

Add this crate to your dependencies section:-

```toml
[dependencies]
reql = "0.0.6-alpha7"
```

Import it in your `main.rs` or `lib.rs`:-

```rust
#[macro_use] extern crate reql;
```

Run ReQL commands:-

```rust
#[macro_use] extern crate reql;

use reql::Client;

fn main() {
    // Create a new ReQL client
    let r = Client::new();
    // Run a command
    let _heroes = r.db("heroes").table(args!("marvel", {read_mode: "outdated"}));
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
