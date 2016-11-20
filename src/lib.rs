//! Rust ReQL command reference
//!
//! Submit issues and pull requests to our [Github
//! repository](https://github.com/rust-rethinkdb/reql).
//!
//! # Accessing ReQL
//!
//! Import this crate:-
//!
//! ```norun
//! extern crate reql;
//! ```
//!
//! ## r
//!
//! > r -> r
//!
//! The top-level ReQL namespace.
//!
//! ### Examples
//!
//! Setup your top-level namespace.
//!
//! ```norun
//! use reql::r;
//! ```
//!
//! [Read more about this command →](constant.r.html)
//!
//! ----------
//!
//! ## connection
//!
//! > r.connection() → builder
//!
//! Create a new connection to the database server. `connection()` returns a builder object with the
//! following methods:
//!
//! - `set_servers()`: the servers to connect to (default `vec!["localhost:28015"]`).
//! - `set_db()`: the default database (default `"test"`).
//! - `set_user()`: the user account to connect as (default `"admin"`).
//! - `set_password()`: the password for the user (default `""`).
//! - `set_retries()`: the number of times to retry a failed command (default `5`).
//! - `connect()`: create a connection pool and connect to all servers with the parameters previously
//! passed to the builder.
//!
//! If the connection cannot be established, an `Error::Driver` will be returned.
//!
//! ### Examples
//!
//! Open a connection using the default host and port, specifying the default database.
//!
//! ```norun
//! r.connection().connect().expect("Failed to connect to the database server");
//! ```
//!
//! [Read more about this command →](command/struct.ConnectOpts.html#method.connect)
//!
//! ----------
//!
//! ## run
//!
//! > query.run() -> stream result
//!
//! Run a query returning a [futures stream receiver].
//!
//! [futures stream receiver]: https://docs.rs/futures/*/futures/stream/struct.Receiver.html
//!
//! ### Examples
//!
//! ```norun
//! let users = try!(r.table("users").run::<User>());
//! let response = users.for_each(|user| {
//!     println!("{:?}", user);
//!     Ok(())
//! });
//! response.wait();
//! ```
//!
//! [Read more about this command →](command/struct.Command.html#method.run)
//!
//! ----------
//!
//! ## run_with_opts
//!
//! > query.run_with_opts(options) -> stream result
//!
//! Run a query specifying certain options and returning a [futures stream receiver].
//!
//! [futures stream receiver]: https://docs.rs/futures/*/futures/stream/struct.Receiver.html
//!
//! ### Examples
//!
//! ```norun
//! let options = r.object(vec![("profile", true)]);
//! let users = try!(r.table("users").run_with_opts::<User>(options));
//! let response = users.for_each(|user| {
//!     println!("{:?}", user);
//!     Ok(())
//! });
//! response.wait();
//! ```
//!
//! [Read more about this command →](command/struct.Command.html#method.run_with_opts)
//!
//! ----------

extern crate ql2;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
extern crate byteorder;
extern crate bufstream;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate quick_error;
extern crate slog_term;
extern crate protobuf;
extern crate scram;
extern crate parking_lot;
extern crate uuid;
extern crate futures;

pub mod command;
pub mod error;

pub use command::{r, Response};
pub use ql2::Command;
