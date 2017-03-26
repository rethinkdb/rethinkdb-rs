mod handshake;
mod pool;

use std::net::{ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr};
use std::{fmt, io, result};

use {Client, Server, Session, IntoArg, Connection, SessionManager, Result};
use super::io_error;
use errors::*;
use reql_io::r2d2;
use reql_io::tokio_core::reactor::Handle;
use reql_io::tokio_core::io::{Codec, EasyBuf};
use reql_io::byteorder::{LittleEndian, ByteOrder};

impl Connect for Client {
    fn connect<T: IntoArg>(&self, args: T) -> Result<Connection> {
        let logger = self.logger.new(o!("command" => "connect"));
        let query = format!("{}.connect({:?}, &handle)", self.query, cfg);
        debug!(logger, "{}", query);
        let mut pool = Connection(Vec::new());
        let remote = handle.remote();
        info!(logger, "creating connection pools...");
        for c in cfg.0 {
            let mut dc = DataCentre(Vec::new());
            for pc in c.servers {
                let logger = self.logger.new(o!(
                        "cluster" => c.name.to_string(),
                        "db" => c.db.to_string(),
                        "user" => c.user.to_string(),
                        ));
                let manager = SessionManager {
                    server: pc.server,
                    remote: remote.clone(),
                    logger: logger,
                };
                match r2d2::Pool::new(pc.pool, manager) {
                    Ok(p) => { dc.0.push(p); }
                    Err(err) => {
                        return Err(From::from(io_error(err)));
                    }
                }
            }
            pool.0.push(dc);
        }
        info!(logger, "connection pools created successfully");
        Ok(pool)
    }
}

impl Config {
    pub fn new() -> Config {
        Config(Vec::new())
    }

    pub fn add_cluster(&mut self, mut cluster: Cluster) -> Result<&mut Config> {
        if cluster.servers.is_empty() {
            let config = r2d2::Config::default();
            cluster.add_server("localhost:28015", config)?;
        }
        self.0.push(cluster);
        Ok(self)
    }
}

impl Cluster {
    pub fn new(name: &str) -> Cluster {
        Cluster {
            name: name.into(),
            servers: Vec::new(),
            db: String::from("test"),
            user: String::from("admin"),
            password: String::new(),
            retries: 5,
            tls: None,
        }
    }
    /// Add a server to the cluster
    pub fn add_server<T: ToSocketAddrs>(&mut self, server: T, config: r2d2::Config<Session, Error>) -> Result<&mut Cluster> {
        let mut addrs = Vec::new();
        for addr in server.to_socket_addrs()? {
            addrs.push(addr);
        }
        if addrs.is_empty() {
            let error = DriverError::Other("no server addresses found".into());
            return Err(From::from(error));
        }
        let server = Server(addrs);
        let cfg = PoolConfig {
            pool: config,
            server: server,
        };
        self.servers.push(cfg);
        Ok(self)
    }
    /// Sets database
    pub fn set_db(&mut self, db: &str) -> &mut Cluster {
        self.db = db.into();
        self
    }
    /// Sets username
    pub fn set_user(&mut self, user: &str) -> &mut Cluster {
        self.user = user.into();
        self
    }
    /// Sets password
    pub fn set_password(&mut self, password: &str) -> &mut Cluster {
        self.password = password.into();
        self
    }
    /// Sets retries
    pub fn set_retries(&mut self, retries: u8) -> &mut Cluster {
        self.retries = retries;
        self
    }
}

impl fmt::Debug for SessionManager {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.server.fmt(formatter)
    }
}

impl Codec for ::Codec {
    type In = (u64, Vec<u8>);
    type Out = (u64, Vec<u8>);

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<(u64, Vec<u8>)>> {
        // At least 12 bytes are required for a frame
        // https://rethinkdb.com/docs/writing-drivers/#receive-responses
        if buf.len() < 12 {
            // We don't yet have a full message
            return Ok(None);
        }

        let id = LittleEndian::read_u64(buf.drain_to(8).as_slice());
        let size = LittleEndian::read_u32(buf.drain_to(4).as_slice()) as usize;
        if buf.len() < size {
            // We don't yet have a full message
            return Ok(None);
        }
        let res = buf.drain_to(size)
            .as_slice()
            .to_owned();
        Ok(Some((id, res)))
    }

    fn encode(&mut self, msg: (u64, Vec<u8>), buf: &mut Vec<u8>) -> io::Result<()> {
        let (id, msg) = msg;

        let mut encoded_id = [0; 8];
        LittleEndian::write_u64(&mut encoded_id, id);

        let mut msg_len = [0; 4];
        LittleEndian::write_u32(&mut msg_len, msg.len() as u32);

        buf.extend(&encoded_id);
        buf.extend(&msg_len);
        buf.extend(&msg);

        Ok(())
    }
}
