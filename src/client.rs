use std::net::SocketAddr;

#[cfg(feature = "tls")]
use native_tls::TlsConnectorBuilder;

#[derive(Debug, Clone, Copy)]
pub struct Client<'a> {
    server: SocketAddr,
    db: &'a str,
    user: &'a str,
    password: &'a str,
    // May be changed to a timeout in future
    // See comment on Default impl
    timeout: u64,
    #[cfg(feature = "tls")]
    tls: Option<&'a mut TlsConnectorBuilder>,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Client {
            server: ([127, 0, 0, 1], 28015).into(),
            db: "test",
            user: "admin",
            password: "",
            // May be changed to a timeout in future
            // See comment on Default impl
            timeout: 5,
            #[cfg(feature = "tls")]
            tls: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config<'a>(Client<'a>);

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config(Client::new())
    }
}

impl<'a> Config<'a> {
    pub fn server(&mut self, addr: SocketAddr) -> &mut Self {
        self.0.server = addr;
        self
    }

    pub fn db(&mut self, name: &'a str) -> &mut Self {
        self.0.db = name;
        self
    }

    pub fn user(&mut self, name: &'a str) -> &mut Self {
        self.0.user = name;
        self
    }

    pub fn password(&mut self, plain: &'a str) -> &mut Self {
        self.0.password = plain;
        self
    }

    pub fn client(&self) -> Client<'a> {
        self.0
    }
}
