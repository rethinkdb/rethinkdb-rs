use std::net::IpAddr;

mod connect;
pub mod run;

#[derive(Debug, Clone)]
pub struct Connect<'a> {
    pub host: IpAddr,
    pub port: u16,
    pub db: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub timeout: u8,
    pub ssl: Option<connect::Ssl<'a>>,
    pub multiplex: bool,
}

#[derive(Debug, Clone)]
pub struct Run {
    pub read_mode: run::ReadMode,
    pub time_format: run::Format,
    pub profile: bool,
    pub durability: run::Durability,
    pub group_format: run::Format,
}
