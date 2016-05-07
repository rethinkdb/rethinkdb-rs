/// Options
#[derive(Debug)]
pub struct Opts {
    pub host: &'static str,
    pub port: u16,
    pub db: &'static str,
    pub user: &'static str,
    pub password: &'static str,
    pub timeout: u16,
    pub ssl: Option<SslCfg>,
}

#[derive(Debug)]
pub struct SslCfg {
    pub ca_certs: &'static str,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts {
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

pub trait Connector {
    type Connection;
    fn close(&self, noreply_wait: bool);
    fn reconnect(&self, noreply_wait: bool) -> Self::Connection;
    // use is a reserved keyword in Rust
    fn use_db(&self, db_name: &str) -> Self::Connection;
}
