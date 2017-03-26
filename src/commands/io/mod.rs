mod pool;
mod handshake;

use std::{io, error};
use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::cmp::Ordering;

use {Client, Config, SessionManager, Server, Result, Connection, Opts, Response, ToArg};
use ql2::proto::{Term, Datum};
use reql_io::r2d2;
use reql_io::ordermap::OrderMap;
use reql_io::uuid::Uuid;
use reql_io::parking_lot::RwLock;
use reql_io::tokio_core::reactor::Remote;
use slog::Logger;

lazy_static! {
    static ref CONFIG: RwLock<OrderMap<Connection, Config>> = RwLock::new(OrderMap::new());
    static ref POOL: RwLock<OrderMap<Connection, r2d2::Pool<SessionManager>>> = RwLock::new(OrderMap::new());
}

pub fn connect<A: ToArg>(client: &Client, args: A) -> Result<Connection> {
    let conn = Connection(Uuid::new_v4());
    let arg = args.to_arg();
    let logger = client.logger.new(o!("command" => "connect"));
    let query = format!("{}.connect({})", client.query, arg.string);
    debug!(logger, "{}", query);
    info!(logger, "creating connection pool...");
    conn.set_config(arg.term, arg.remote, logger.clone())?;
    conn.maintain();
    let config = r2d2::Config::default();
    let session = SessionManager(conn);
    let r2d2 = r2d2::Pool::new(config, session).map_err(|err| io_error(err))?;
    conn.set_pool(r2d2);
    info!(logger, "connection pool created successfully");
    Ok(conn)
}

pub fn run<A: ToArg>(client: &Client, args: A) -> Result<Response> {
    unimplemented!();
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
    fn set_config(&self, mut term: Term, remote: Option<Remote>, logger: Logger) -> Result<()> {
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
