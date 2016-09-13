//! ReQL Traits and Error Types
//!
//! These are the common traits and [error types] returned by ReQL drivers.
//!
//! [error types]: https://www.rethinkdb.com/docs/error-types/

mod errors;
mod traits;

pub use errors::*;
pub use traits::*;

/// Options
#[derive(Debug, Clone)]
pub struct ConnectOpts {
    pub host: &'static str,
    pub port: u16,
    pub db: &'static str,
    pub user: &'static str,
    pub password: &'static str,
    pub timeout: u16,
    pub ssl: Option<SslCfg>,
}

#[derive(Debug, Clone)]
pub struct SslCfg {
    pub ca_certs: &'static str,
}

impl Default for ConnectOpts {
    fn default() -> ConnectOpts {
        ConnectOpts {
            host: "localhost",
            port: 28015,
            db: "test",
            user: "admin",
            password: "",
            timeout: 20,
            ssl: None,
        }
    }
}

impl IntoConnectOpts for ConnectOpts {
    fn into(self) -> ConnectOpts {
        self
    }
}
