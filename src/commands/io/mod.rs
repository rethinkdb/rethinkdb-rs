mod pool;
mod request;
mod handshake;

use std::{io, error};
use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::cmp::Ordering;

use {Client, Config, SessionManager, Server, Result, Connection, Opts, Response, ResponseValue, IntoArg, Run};
use ql2::proto::{Term, Datum};
use r2d2;
use ordermap::OrderMap;
use uuid::Uuid;
use parking_lot::RwLock;
use tokio_core::reactor::Remote;
use slog::Logger;
use serde::Deserialize;
use errors::*;
use futures::sync::mpsc::{self, Sender};

const CHANNEL_SIZE: usize = 1024 * 1024;

lazy_static! {
    static ref CONFIG: RwLock<OrderMap<Connection, Config>> = RwLock::new(OrderMap::new());
    static ref POOL: RwLock<OrderMap<Connection, r2d2::Pool<SessionManager>>> = RwLock::new(OrderMap::new());
}

struct Request<T: Deserialize + Send + 'static> {
    term: Term,
    opts: Term,
    conn: Connection,
    tx: Sender<Result<ResponseValue<T>>>,
}

pub fn connect<A: IntoArg>(client: &Client, args: A) -> Result<Connection> {
    if let Err(ref error) = client.term {
        return Err(error.clone());
    }
    let arg = args.into_arg();
    let aterm = arg.term?;
    let conn = Connection(Uuid::new_v4());
    let logger = client.logger.new(o!("command" => "connect"));
    let query = format!("{}.connect({})", client.query, arg.string);
    debug!(logger, "{}", query);
    info!(logger, "creating connection pool...");
    match arg.remote {
        Some(remote) => conn.set_config(aterm, remote, logger.clone())?,
        None => { return Err(io_error("a futures handle is required for `connect`"))?; }
    }
    conn.maintain();
    let config = r2d2::Config::default();
    let session = SessionManager(conn);
    let r2d2 = r2d2::Pool::new(config, session).map_err(|err| io_error(err))?;
    conn.set_pool(r2d2);
    info!(logger, "connection pool created successfully");
    Ok(conn)
}

impl<A: IntoArg> Run<A> for Client {
    fn run<T: Deserialize + Send + 'static>(&self, args: A) -> Result<Response<T>> {
        let cterm = match self.term {
            Ok(ref term) => term.clone(),
            Err(ref error) => { return Err(error.clone()); }
        };
        let arg = args.into_arg();
        let aterm = arg.term?;
        let logger = self.logger.new(o!("command" => "run"));
        let query = format!("{}.run({})", self.query, arg.string);
        debug!(logger, "{}", query);
        let conn = match arg.pool {
            Some(pool) => pool,
            None => {
                let msg = String::from("`run` requires a connection");
                return Err(DriverError::Other(msg))?;
            }
        };
        let (tx, rx) = mpsc::channel::<Result<ResponseValue<T>>>(CHANNEL_SIZE);
        let remote = match CONFIG.read().get(&conn) {
            Some(opts) => opts.remote.clone(),
            None => { return Err(io_error("a tokio handle is required"))?; }
        };
        remote.spawn(move |_| {
            let request = Request {
                term: cterm,
                opts: aterm,
                conn: conn,
                tx: tx,
            };
            request.submit()
        });
        Ok(rx)
    }
}

fn io_error<T>(err: T) -> io::Error
    where T: Into<Box<error::Error + Send + Sync>>
{
    io::Error::new(io::ErrorKind::Other, err)
}

fn take_string(val: Vec<Datum>) -> String {
    for mut datum in val {
        return datum.take_r_str();
    }
    String::new()
}

impl Default for Opts {
    fn default() -> Opts {
        Opts {
            db: "test".into(),
            user: "admin".into(),
            password: String::new(),
            retries: 5,
            tls: None,
        }
    }
}

fn find_datum(mut term: Term) -> Vec<Datum> {
    let mut res = Vec::new();
    if term.has_datum() {
        res.push(term.take_datum());
    } else {
        for term in term.take_args().into_vec() {
            for datum in find_datum(term) {
                res.push(datum);
            }
        }
    }
    res
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
    fn set_config(&self, mut term: Term, remote: Remote, logger: Logger) -> Result<()> {
        let mut cluster = Vec::new();
        let mut hosts = Vec::new();
        let mut opts = Opts::default();

        let optargs = term.take_optargs().into_vec();
        for mut arg in optargs {
            let key = arg.take_key();
            let val = find_datum(arg.take_val());

            if key == "db" { opts.db = take_string(val); }
            else if key == "user" { opts.user = take_string(val); }
            else if key == "password" { opts.password = take_string(val); }
            else if key == "servers" { hosts.extend(val.into_iter().map(|datum| take_string(vec![datum]))); }
        }

        if hosts.is_empty() {
            hosts.push("localhost".into());
        }

        for host in hosts {
            let addresses = host.to_socket_addrs().or_else(|_| {
                    let host = format!("{}:{}", host, 28015);
                    host.to_socket_addrs()
                })?;
            cluster.push(Server {
                addresses: addresses.collect(),
                latency: Duration::from_millis(u64::max_value()),
            });
        }

        CONFIG.write().insert(*self, Config {
            cluster: cluster,
            opts: opts,
            remote: remote,
            logger: logger,
        });

        Ok(())
    }

    fn maintain(&self) {
        let mut config = self.config();
        for mut server in config.cluster.iter_mut() {
            for address in server.addresses.iter() {
                let start = Instant::now();
                if let Ok(_) = TcpStream::connect(address) {
                    server.latency = start.elapsed();
                    break;
                }
            }
        }
        config.cluster.sort();
        CONFIG.write().insert(*self, config);
    }

    fn config(&self) -> Config {
        CONFIG.read().get(self).unwrap().clone()
    }

    fn set_pool(&self, pool: r2d2::Pool<SessionManager>) {
        POOL.write().insert(*self, pool);
    }
}
