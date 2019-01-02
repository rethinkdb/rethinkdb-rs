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
    #[doc(hidden)]
    pub fn multiplex(mut self, multiplex: bool) -> Self {
        self.multiplex = multiplex;
        self
    }
}
