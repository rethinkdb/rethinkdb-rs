use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Opts {
    pub(super) host: IpAddr,
    pub(super) port: u16,
    pub(super) db: String,
    pub(super) user: String,
    pub(super) password: String,
    pub(super) timeout: u8,
    pub(super) multiplex: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            host: [127, 0, 0, 1].into(),
            port: 28015,
            db: "test".to_owned(),
            user: "admin".to_owned(),
            password: String::new(),
            timeout: 20,
            multiplex: true,
        }
    }
}

impl Opts {
    /// The host to connect to (default `127.0.0.1`)
    pub fn host<T>(mut self, host: T) -> Self
    where
        T: Into<IpAddr>,
    {
        self.host = host.into();
        self
    }

    /// The port to connect on (default `28015`)
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// The default database (default `test`)
    pub fn db(mut self, db: &str) -> Self {
        self.db = db.to_owned();
        self
    }

    /// The user account to connect as (default `admin`)
    pub fn user(mut self, user: &str) -> Self {
        self.user = user.to_owned();
        self
    }

    /// The password for the user account to connect as (default `""`, empty)
    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_owned();
        self
    }

    #[doc(hidden)]
    pub fn multiplex(mut self, multiplex: bool) -> Self {
        self.multiplex = multiplex;
        self
    }
}
