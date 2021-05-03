use blocking::unblock;
use futures::lock::Mutex;
use futures::TryStreamExt;
use futures_timer::Delay;
use log::trace;
use mobc::{async_trait, Manager, Pool};
use reql::cmd::connect::Options;
use reql::cmd::run::{self, Arg};
use reql::{r, Client, Connection, Error, Query, Result};
use reql_types::{Change, ServerStatus};
use std::cmp::Ordering;
use std::io;
use std::net::{IpAddr, TcpStream};
use std::ops::Deref;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[async_trait]
pub trait GetConn {
    async fn get_conn(&self) -> Result<PooledConnection>;
}

#[async_trait]
impl GetConn for Pool<ReqlConnectionManager> {
    async fn get_conn(&self) -> Result<PooledConnection> {
        Ok(PooledConnection {
            conn: self.get().await.map_err(to_reql)?,
        })
    }
}

pub struct PooledConnection {
    conn: mobc::Connection<ReqlConnectionManager>,
}

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl AsRef<Connection> for PooledConnection {
    fn as_ref(&self) -> &Connection {
        self.deref()
    }
}

impl<'a> Arg<'a> for &'a PooledConnection {
    fn into_run_opts(self) -> (&'a Connection, run::Options) {
        self.deref().into_run_opts()
    }
}

#[derive(Debug, Clone, Eq, Hash)]
struct Server {
    name: String,
    addresses: Vec<IpAddr>,
    port: u16,
    latency: Duration,
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
        self.name == other.name
    }
}

impl Server {
    fn from_status(status: ServerStatus) -> Self {
        let network = status.network;
        let addresses = network.canonical_addresses.into_iter().map(move |x| x.host);
        Self {
            name: status.name,
            addresses: addresses.collect(),
            port: network.reql_port,
            latency: Duration::from_millis(u64::MAX),
        }
    }
}

#[derive(Clone)]
pub struct ReqlConnectionManager {
    opts: Options,
    servers: Arc<Mutex<Vec<Server>>>,
    pool: Option<Pool<Self>>,
}

impl ReqlConnectionManager {
    pub fn new(opts: Options) -> Self {
        Self {
            opts,
            servers: Arc::new(Mutex::new(Vec::new())),
            pool: None,
        }
    }

    pub async fn discover_hosts(&mut self) -> Result<()> {
        self.pool = Some(Pool::builder().max_open(2).build(self.clone()));
        *self.servers.lock().await = self.get_servers().await?;
        let manager = self.clone();
        self.spawn_task(async move {
            let mut wait = 0;
            loop {
                if let Err(error) = manager.listen_for_hosts(&mut wait).await {
                    trace!(
                        "listening for host changes; error: {}, wait: {}s",
                        error,
                        wait
                    );
                    Delay::new(Duration::from_secs(wait)).await;
                    wait = 300.min(wait + 1);
                }
            }
        });
        Ok(())
    }

    async fn listen_for_hosts(&self, wait: &mut u64) -> Result<()> {
        let conn = self.pool.as_ref().unwrap().get_conn().await?;
        let mut query = server_status()
            .changes(())
            .run::<_, Change<ServerStatus, ServerStatus>>(&conn);
        while let Some(_) = query.try_next().await? {
            *self.servers.lock().await = self.get_servers().await?;
            *wait = 0;
        }
        Ok(())
    }

    async fn get_servers(&self) -> Result<Vec<Server>> {
        let mut servers = Vec::new();
        let conn = self.pool.as_ref().unwrap().get_conn().await?;
        let mut query = server_status().run(&conn);
        while let Some(status) = query.try_next().await? {
            servers.push(Server::from_status(status));
        }
        set_latency(&mut servers).await;
        servers.sort();
        Ok(servers)
    }
}

async fn set_latency(servers: &mut Vec<Server>) {
    for server in servers {
        let port = server.port;
        for (i, host) in server.addresses.iter().enumerate() {
            let host = *host;
            let latency = unblock(move || {
                let start = Instant::now();
                if let Ok(_) = TcpStream::connect((host, port)) {
                    return Some(start.elapsed());
                }
                None
            })
            .await;
            if let Some(latency) = latency {
                if latency > server.latency || i == 0 {
                    server.latency = latency;
                }
            }
        }
    }
}

fn server_status() -> Query {
    r.db("rethinkdb").table("server_status")
}

#[async_trait]
impl Manager for ReqlConnectionManager {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection> {
        let opts = &self.opts;
        let servers = &self.servers.lock().await;
        if servers.is_empty() {
            trace!(
                "no discovered servers; host: {}, port: {}",
                opts.host,
                opts.port
            );
            return Ok(r.connect(opts.clone()).await?);
        } else {
            for server in servers.iter() {
                for host in &server.addresses {
                    trace!(
                        "discovered server {}; host: {}, port: {}",
                        server.name,
                        host,
                        server.port
                    );
                    let addr = (*host, server.port);
                    if let Ok(conn) = r.connect((addr, opts.clone())).await {
                        return Ok(conn);
                    }
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "no RethinkDB servers available",
        )
        .into())
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection> {
        let msg = 200;
        match r.expr(msg).run(&conn).try_next().await? {
            Some(res) => verify(res, msg)?,
            None => {
                return Err(Client::ConnectionBroken.into());
            }
        }
        Ok(conn)
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_broken()
    }
}

fn verify(res: u32, msg: u32) -> Result<()> {
    if res != msg {
        return Err(Client::ConnectionBroken.into());
    }
    Ok(())
}

fn to_reql(error: mobc::Error<Error>) -> Error {
    match error {
        mobc::Error::Inner(error) => error,
        mobc::Error::Timeout => io::Error::from(io::ErrorKind::TimedOut).into(),
        mobc::Error::BadConn => Client::ConnectionBroken.into(),
    }
}
