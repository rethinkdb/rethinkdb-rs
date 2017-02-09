use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::{fmt, io, result};

use {Client, Config, Connection, InnerConfig, Pool, ConnectionManager, Result, Server};
use super::{Connect, io_error};
use errors::Error;
use reql_io::r2d2;
use reql_io::tokio_core::reactor::Handle;
use reql_io::tokio_core::io::{Codec, EasyBuf};
use reql_io::byteorder::{LittleEndian, ByteOrder};

impl Connect for Client {
    fn connect(&self, cfg: Config, handle: &Handle) -> Result<Pool> {
        let logger = self.logger.new(o!("command" => "connect"));
        let query = format!("{}.connect({:?}, &handle)", self.query, cfg);
        debug!(logger, "{}", query);
        let mut pool = Pool(Vec::new());
        let remote = handle.remote();
        for c in cfg.0 {
            let manager = ConnectionManager {
                server: c.server,
                remote: remote.clone(),
                logger: logger.clone(),
            };
            match r2d2::Pool::new(c.pool, manager) {
                Ok(p) => { pool.0.push(p); }
                Err(err) => {
                    return Err(From::from(io_error(err)));
                }
            }
        }
        Ok(pool)
    }
}

impl Config {
    pub fn new() -> Config {
        Config(Vec::new())
    }

    pub fn add_server(&mut self, server: Server, pool: r2d2::Config<Connection, Error>) -> &mut Config {
        let cfg = InnerConfig {
            pool: pool,
            server: server,
        };
        self.0.push(cfg);
        self
    }
}

impl Server {
    pub fn new(name: &str) -> Server {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let address = SocketAddr::new(localhost, 28015);
        Server {
            name: name.into(),
            addresses: vec![address],
            db: String::from("test"),
            user: String::from("admin"),
            password: String::new(),
            retries: 5,
            tls: None,
        }
    }
}

impl fmt::Debug for ConnectionManager {
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
