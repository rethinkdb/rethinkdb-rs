use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::{fmt, result};

use {Client, Config, InnerConfig, Connection, ConnectionManager, Result, Opts};
use reql_io::r2d2;
use reql_io::tokio_core::reactor::Handle;
use super::{Connect, io_error};

impl Connect for Client {
    fn connect(&self, cfg: Config, handle: &Handle) -> Result<Connection> {
        let logger = self.logger.new(o!("command" => "connect"));
        let query = format!("{}.connect({:?}, &handle)", self.query, cfg);
        debug!(logger, "{}", query);
        let mut connection = Connection(Vec::new());
        let remote = handle.remote();
        for c in cfg.0 {
            let manager = ConnectionManager {
                opts: c.opts,
                remote: remote.clone(),
            };
            match r2d2::Pool::new(c.pool, manager) {
                Ok(pool) => { connection.0.push(pool); }
                Err(err) => {
                    return Err(From::from(io_error(err)));
                }
            }
        }
        Ok(connection)
    }
}

impl Default for Opts {
    fn default() -> Opts {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Opts {
            addresses: vec![SocketAddr::new(localhost, 28015)],
            db: "test",
            user: "admin",
            password: "",
            retries: 5,
            tls: None,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        let cfg = InnerConfig {
            pool: Default::default(),
            opts: Default::default(),
        };
        Config(vec![cfg])
    }
}

impl fmt::Debug for ConnectionManager {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.opts.fmt(formatter)
    }
}
