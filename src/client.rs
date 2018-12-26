use std::net::SocketAddr;

use crate::{
    Result,
    conn::Connection,
};

#[derive(Debug, Clone, Copy)]
pub struct Client<'a> {
    server: SocketAddr,
    db: &'a str,
    user: &'a str,
    pass: &'a str,
    timeout: u64,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Client {
            server: ([127, 0, 0, 1], 28015).into(),
            db: "test",
            user: "admin",
            pass: "",
            timeout: 5,
        }
    }

    pub fn config(self) -> Config<'a> {
        Config {
            client: self,
        }
    }

    pub async fn connect(&mut self) -> Result<Connection<'a>> {
        await!(Connection::new(*self))
    }
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    client: Client<'a>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            client: Client::new(),
        }
    }
}

impl<'a> Config<'a> {
    pub fn set_server(&mut self, addr: SocketAddr) -> &mut Self {
        self.client.server = addr;
        self
    }

    pub fn server(&self) -> &SocketAddr {
        &self.client.server
    }

    pub fn set_db(&mut self, name: &'a str) -> &mut Self {
        self.client.db = name;
        self
    }

    pub fn db(&self) -> &str {
        &self.client.db
    }

    pub fn set_user(&mut self, name: &'a str) -> &mut Self {
        self.client.user = name;
        self
    }

    pub fn user(&self) -> &str {
        &self.client.user
    }

    pub fn set_password(&mut self, plain: &'a str) -> &mut Self {
        self.client.pass = plain;
        self
    }

    pub fn password(&self) -> &str {
        &self.client.pass
    }

    pub fn client(&self) -> Client<'a> {
        self.client
    }
}
