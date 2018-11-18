mod pool;
mod request;
mod handshake;

use crate::{
    Client, InnerConfig, Config, Connection, IntoArg,
    Opts, DEFAULT_PORT, Request, Response, Result, Run,
    Server, Session, SessionManager,
    errors::*,
};
use crate::Document;
use r2d2;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;
use parking_lot::RwLock;
use protobuf::ProtobufEnum;
use ql2::proto::Query_QueryType as QueryType;
use serde::de::DeserializeOwned;
use std::{error};
use std::cmp::Ordering;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use uuid::Uuid;
use lazy_static::lazy_static;
use futures::sync::mpsc;
use futures::{Async, Poll, Stream};

lazy_static! {
    static ref CONFIG: RwLock<IndexMap<Connection, InnerConfig>> = RwLock::new(IndexMap::new());
    static ref POOL: RwLock<IndexMap<Connection, r2d2::Pool<SessionManager>>> = RwLock::new(IndexMap::new());
}

const CHANNEL_SIZE: usize = 1024;

pub fn connect<'a>(client: &Client, cfg: Config<'a>) -> Result<Connection> {
    if let Err(ref error) = client.term {
        return Err(error.clone());
    }
    let conn = Connection(Uuid::new_v4());
    let query = format!("{}.connect({:?})", client.query, cfg);
    log::debug!("{}", query);
    log::info!("creating connection pool...");
    conn.set_config(cfg)?;
    conn.set_latency()?;
    let session = SessionManager(conn);
    let r2d2 = r2d2::Pool::builder()
        .max_size(144)
        .idle_timeout(Some(Duration::from_secs(30)))
        .max_lifetime(Some(Duration::from_secs(150)))
        .min_idle(Some(5))
        .connection_timeout(Duration::from_secs(3))
        .build(session)?;
    conn.set_pool(r2d2);
    log::info!("connection pool created successfully");
    Ok(conn)
}

impl<A: IntoArg + std::fmt::Debug> Run<A> for Client {
    fn run<T: DeserializeOwned + Send + std::fmt::Debug + 'static>(&self, args: A) -> Result<Response<T>> {
        let cterm = match self.term {
            Ok(ref term) => term.clone(),
            Err(ref error) => {
                return Err(error.clone());
            }
        };
        let arg = args.into_arg();
        let aterm = arg.term?;
        let query = format!("{}.run({})", self.query, arg.string);
        log::debug!("{}", query);
        let conn = match arg.pool {
            Some(conn) => conn.clone(),
            None => {
                let msg = String::from("`run` requires a connection");
                return Err(DriverError::Other(msg))?;
            }
        };
        let pool = match POOL.read().get(&conn) {
            Some(pool) => pool.clone(),
            None => {
                let msg = String::from("bug: connection not in POOL");
                return Err(DriverError::Other(msg))?;
            }
        };
        let cfg = match CONFIG.read().get(&conn) {
            Some(cfg) => cfg.clone(),
            None => {
                return Err(io_error("a tokio handle is required"))?;
            }
        };
        //let (tx, rx) = mpsc::channel();
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        // @TODO spawning a thread per query is less than ideal. Ideally we will
        // need first class support for Tokio to get rid of this.
        let _ = ::std::thread::Builder::new()
            .name("submit".into())
            //.stack_size(78048)
            .spawn(move || {
            let req = Request {
                term: cterm,
                opts: aterm,
                pool: pool,
                cfg: cfg,
                tx: tx,
                write: true,
                retry: false,
            };
            req.submit();
        });
        Ok(Response {
               done: false,
               rx: rx,
           })
    }
}

impl<T: DeserializeOwned + Send + std::fmt::Debug> Stream for Response<T> {
    type Item = Option<Document<T>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.done {
            return Ok(Async::Ready(None));
        }
        match self.rx.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(Some(res))) => {
                match res {
                    Ok(data) => Ok(Async::Ready(Some(data))),
                    Err(error) => Err(error),
                }
            }
            Ok(Async::Ready(None)) => {
                self.done = true;
                Ok(Async::Ready(None))
            }
            Err(_) => {
                self.done = true;
                let msg = String::from("an error occured while processing the stream");
                Err(DriverError::Other(msg))?
            }
        }
    }
}

fn io_error<T>(err: T) -> io::Error
    where T: Into<Box<error::Error + Send + Sync>>
{
    io::Error::new(io::ErrorKind::Other, err)
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        let ip4 = format!("127.0.0.1:{}", DEFAULT_PORT);
        let ip6 = format!("[::1]:{}", DEFAULT_PORT);
        Config {
            servers: vec![
                ip4.parse().unwrap(),
                ip6.parse().unwrap(),
            ],
            db: "test",
            user: "admin",
            password: "",
            // @TODO number of retries doesn't mean much
            // let's use a timeout instead and make it an
            // option in both connect and run. The connect
            // one will be the user default and the run one
            // will have the highest precedence. Also let's
            // call it `retry_timeout` to communicate clearly
            // what it does.
            retries: 5,
            #[cfg(feature = "tls")]
            tls: None,
        }
    }
}

impl Ord for Server {
    fn cmp(&self, other: &Server) -> Ordering {
        self.latency.cmp(&other.latency)
    }
}

impl PartialOrd for Server {
    fn partial_cmp(&self, other: &Server) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Server {
    fn eq(&self, other: &Server) -> bool {
        self.latency == other.latency
    }
}

impl Connection {
    fn set_config<'a>(&self, cfg: Config<'a>) -> Result<()> {
        let opts = Opts {
            db: cfg.db.into(),
            user: cfg.user.into(),
            password: cfg.password.into(),
            retries: cfg.retries,
            reproducible: false,
            #[cfg(feature = "tls")]
            tls: cfg.tls,
        };

        let mut cluster = IndexMap::new();
        let host = String::from("initial-cluster");
        let server = Server::new(&host, cfg.servers);
        cluster.insert(host, server);

        CONFIG
            .write()
            .insert(*self,
                    InnerConfig {
                        cluster: cluster,
                        opts: opts,
                    });

        Ok(())
    }


    fn set_latency(&self) -> Result<()> {
        match CONFIG.write().get_mut(self) {
            Some(ref mut config) => {
                for server in config.cluster.values_mut() {
                    server.set_latency();
                }
                Ok(())
            }
            None => {
                let msg = String::from("conn.set_latency() called before setting configuration");
                Err(DriverError::Other(msg))?
            }
        }
    }

    fn config(&self) -> InnerConfig {
        CONFIG.read().get(self).unwrap().clone()
    }

    fn set_pool(&self, pool: r2d2::Pool<SessionManager>) {
        POOL.write().insert(*self, pool);
    }
}

impl Server {
    fn new(host: &str, addresses: Vec<SocketAddr>) -> Server {
        Server {
            name: host.to_string(),
            addresses: addresses,
            latency: Duration::from_millis(u64::max_value()),
        }
    }

    fn set_latency(&mut self) {
        for address in self.addresses.iter() {
            let start = Instant::now();
            if let Ok(_) = TcpStream::connect(address) {
                self.latency = start.elapsed();
                break;
            }
        }
    }
}

fn write_query(conn: &mut Session, query: &str) -> Result<()> {
    let query = query.as_bytes();
    let token = conn.id;
    if let Err(error) = conn.stream.write_u64::<LittleEndian>(token) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.write_u32::<LittleEndian>(query.len() as u32) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.write_all(query) {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    if let Err(error) = conn.stream.flush() {
        conn.broken = true;
        return Err(io_error(error))?;
    }
    Ok(())
}

fn read_query(conn: &mut Session) -> Result<Vec<u8>> {
    let _ = match conn.stream.read_u64::<LittleEndian>() {
        Ok(token) => token,
        Err(error) => {
            conn.broken = true;
            return Err(io_error(error))?;
        }
    };
    let len = match conn.stream.read_u32::<LittleEndian>() {
        Ok(len) => len,
        Err(error) => {
            conn.broken = true;
            return Err(io_error(error))?;
        }
    };
    let mut resp = vec![0u8; len as usize];
    if let Err(error) = conn.stream.read_exact(&mut resp) {
        conn.broken = true;
        return Err(io_error(error))?;
    }

    Ok(resp)
}

fn wrap_query(query_type: QueryType, query: Option<String>, options: Option<String>) -> String {
    let mut qry = format!("[{}", query_type.value());
    if let Some(query) = query {
        qry.push_str(&format!(",{}", query));
    }
    if let Some(options) = options {
        qry.push_str(&format!(",{}", options));
    }
    qry.push_str("]");
    qry
}
