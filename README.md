# RethinkDB Driver

This is a [RethinkDB] driver written in [Rust].

[RethinkDB]: https://www.rethinkdb.com
[Rust]: https://www.rust-lang.org

[![Build Status](https://travis-ci.org/rust-rethinkdb/reql.svg?branch=master)](https://travis-ci.org/rust-rethinkdb/reql) [![Latest Version](https://img.shields.io/crates/v/reql.svg)](https://crates.io/crates/reql) [![Docs](https://docs.rs/reql/badge.svg)](https://docs.rs/reql)

*Note:* While this driver is already usable in the current state, the API is not yet stable and many commands are not yet implemented. I recommend you pin to specific versions if you have to code against it. Also kindly submit an issue or pull request if the command you want is missing.

## Getting Started

Add this crate to your dependencies section:-

```text
[dependencies]
reql = "0.0.5"
```

Import it in your `main.rs` or `lib.rs`:-

```rust,ignore
extern crate reql;

use reql::prelude::*;
use reql::r;
```

Create a connection pool and connect to your database server(s) in your `main` or similar function if creating a library:-

```rust
extern crate reql;

use reql::r;

fn main() {
    r.connection().connect().expect("Failed to connect to the database server");
}
```

Run ReQL commands:-

```rust,ignore
extern crate reql;

use reql::prelude::*;
use reql::r;

fn print_users() -> Result<(), Error> {
    let users = try!(r.table("users").run::<User>());
    let response = users.for_each(|user| {
        println!("{:?}", user);
        Ok(())
    });
    for v in response.wait() {
        // Do something with v
    }
}

fn main() {
    r.connection().connect().expect("Failed to connect to the database server");
}
```
