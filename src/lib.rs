//! ReQL Traits and Error Types
//!
//! These are the common traits and [error types] returned by ReQL drivers.
//!
//! [error types]: https://www.rethinkdb.com/docs/error-types/

mod errors;
mod traits;

pub use errors::*;
pub use traits::*;
