use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Opts {
    pub(super) host: IpAddr,
    pub(super) port: u16,
    pub(super) db: String,
    pub(super) user: String,
    pub(super) password: String,
    pub(super) timeout: u8,
    // Buffer size of the futures `mpsc::channel`. This is used for connection
    // multiplexing. A size of 0 disables multiplexing altogether.
    pub(super) buffer: usize,
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
            //buffer: 1024,
            buffer: 0,
        }
    }
}

impl Opts {
    pub fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }
}
