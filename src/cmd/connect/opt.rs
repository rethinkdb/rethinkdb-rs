use {crate::cmd::make_builder, std::net::IpAddr};

#[derive(Debug, Clone, Copy)]
pub struct Opts<'a> {
    pub(crate) host: IpAddr,
    pub(crate) port: u16,
    pub(crate) db: &'a str,
    pub(crate) user: &'a str,
    pub(crate) password: &'a str,
    pub(crate) timeout: u8,
}

impl<'a> Opts<'a> {
    make_builder!();

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
        }
    }
}

impl<'a> From<()> for Opts<'a> {
    fn from(_: ()) -> Self {
        Default::default()
    }
}
