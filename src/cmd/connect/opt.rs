use std::net::IpAddr;

#[derive(Debug, Clone, Copy)]
pub struct Opts<'a> {
    pub(super) host: IpAddr,
    pub(super) port: u16,
    pub(super) db: &'a str,
    pub(super) user: &'a str,
    pub(super) password: &'a str,
    pub(super) timeout: u8,
    pub(super) multiplex: bool,
}

impl<'a> Opts<'a> {
    /// Start building the options
    pub fn builder() -> Self {
        Default::default()
    }

    /// The host to connect to (default `127.0.0.1`)
    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<IpAddr>,
    {
        self.host = host.into();
        self
    }

    /// The port to connect on (default `28015`)
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    /// The default database (default `test`)
    pub fn db(&mut self, db: &'a str) -> &mut Self {
        self.db = db;
        self
    }

    /// The user account to connect as (default `admin`)
    pub fn user(&mut self, user: &'a str) -> &mut Self {
        self.user = user;
        self
    }

    /// The password for the user account to connect as (default `""`, empty)
    pub fn password(&mut self, password: &'a str) -> &mut Self {
        self.password = password;
        self
    }

    #[doc(hidden)]
    pub fn multiplex(&mut self, multiplex: bool) -> &mut Self {
        self.multiplex = multiplex;
        self
    }

    // Finalise the options
    pub fn build(&self) -> Self {
        *self
    }
}

impl<'a> Default for Opts<'a> {
    fn default() -> Self {
        Self {
            host: [127, 0, 0, 1].into(),
            port: 28015,
            db: "",
            user: "admin",
            password: "",
            timeout: 20,
            multiplex: true,
        }
    }
}

impl<'a> From<()> for Opts<'a> {
    fn from(_: ()) -> Self {
        Default::default()
    }
}
