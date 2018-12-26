use std::{
    net::SocketAddr,
    time::Duration,
};

use crate::{
    Result,
    conn::Connection,
};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) server: SocketAddr,
    pub(crate) user: String,
    pub(crate) pass: String,
    pub(crate) timeout: Duration,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            server: ([127, 0, 0, 1], 28015).into(),
            user: "admin".to_owned(),
            pass: String::new(),
            timeout: Duration::from_secs(5),
        }
    }
}

impl Client {
    pub fn new(server: SocketAddr, user: impl Into<String>, pass: impl Into<String>, timeout: Duration) -> Self {
        Client {
            server, timeout,
            user: user.into(),
            pass: pass.into(),
        }
    }

    pub async fn connect(&self) -> Result<Connection> {
        await!(Connection::new(self))
    }
}
