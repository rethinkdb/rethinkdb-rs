mod pool;
mod request;
mod handshake;

use std::{error, thread};
use std::io::{self, Write, Read};
use std::net::{ToSocketAddrs, SocketAddr};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::cmp::Ordering;

use {Client, Config, SessionManager, Server,
        Result, Connection, Opts, Request, Response,
        IntoArg, Session, Run, ResponseValue,
};
use ql2::proto::{Term, Datum};
use r2d2;
use ordermap::OrderMap;
use uuid::Uuid;
use parking_lot::RwLock;
use tokio_core::reactor::Remote;
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use slog::Logger;
use serde::de::DeserializeOwned;
use errors::*;
use futures::{Stream, Sink};
use futures::sync::mpsc;
use protobuf::ProtobufEnum;
use ql2::proto::Query_QueryType as QueryType;
use structs::{Change, ServerStatus};

lazy_static! {
    static ref CONFIG: RwLock<OrderMap<Connection, Config>> = RwLock::new(OrderMap::new());
    static ref POOL: RwLock<OrderMap<Connection, r2d2::Pool<SessionManager>>> = RwLock::new(OrderMap::new());
}

const CHANNEL_SIZE: usize = 1024;

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
    conn.set_latency()?;
    let config = r2d2::Config::default();
    let session = SessionManager(conn);
    let r2d2 = r2d2::Pool::new(config, session).map_err(|err| io_error(err))?;
    conn.set_pool(r2d2);
    info!(logger, "connection pool created successfully");
    conn.maintain();
    Ok(conn)
}

impl<A: IntoArg> Run<A> for Client {
    fn run<T: DeserializeOwned + Send + 'static>(&self, args: A) -> Result<Response<T>> {
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
            None => { return Err(io_error("a tokio handle is required"))?; }
        };
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        //let remote = cfg.remote.clone();
        // @TODO spawning a thread per query is less than ideal. Ideally we will
        // need first class support for Tokio to get rid of this.
        ::std::thread::spawn(move || {
            let req = Request {
                term: cterm,
                opts: aterm,
                pool: pool,
                cfg: cfg,
                tx: tx,
                write: true,
                retry: false,
                logger: logger,
            };
            req.submit();
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
        let mut cluster = OrderMap::new();
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
            let server = Server::new(&host, addresses.collect());
            cluster.insert(host, server);
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
        self.reset_cluster();
        let conn = *self;
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        thread::spawn(move || {
            let r = Client::new();
            let query = r.db("rethinkdb").table("server_status").changes().with_args(args!({include_initial: true}));
            loop {
                let changes = query.run::<Change<ServerStatus, ServerStatus>>(conn).unwrap();
                for change in changes.wait() {
                    match change {
                        Ok(Ok(Some(ResponseValue::Expected(change)))) => {
                            if let Some(ref mut config) = CONFIG.write().get_mut(&conn) {
                                let cluster = &mut config.cluster;
                                if let Some(status) = change.new_val {
                                    let mut addresses = Vec::new();
                                    for addr in status.network.canonical_addresses {
                                        let socket = SocketAddr::new(addr.host, status.network.reql_port);
                                        addresses.push(socket);
                                    }
                                    let server = Server::new(&status.name, addresses);
                                    cluster.insert(server.name.to_owned(), server);
                                    let _ = tx.clone().send(());
                                } else if let Some(status) = change.old_val {
                                    cluster.remove(&status.name);
                                }
                            }
                        }
                        Ok(Ok(res)) => {
                            println!("unexpected response from server: {:?}", res);
                        }
                        Ok(Err(error)) => {
                            println!("{:?}", error);
                        }
                        Err(_) => {
                            println!("an error occured while processing the stream");
                        }
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        // wait for at least one database result before continuing
        let _ = rx.wait();
    }

    fn reset_cluster(&self) {
        if let Some(ref mut config) = CONFIG.write().get_mut(self) {
            config.cluster = OrderMap::new();
        }
    }

    fn set_latency(&self) -> Result<()> {
        match CONFIG.write().get_mut(self) {
            Some(ref mut config) => {
                for mut server in config.cluster.values_mut() {
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

    fn config(&self) -> Config {
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

fn wrap_query(query_type: QueryType,
              query: Option<String>,
              options: Option<String>)
-> String {
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
