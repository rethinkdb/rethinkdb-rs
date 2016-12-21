#![allow(dead_code)]

use std::net::TcpStream;

use parking_lot::RwLock;
use errors::*;
use conn::{
    ConnectionOpts,
    Connection,
    ReqlConnection,
    TlsCfg,
    Session,
};
use ::{Client, Command, Result, set_config, config, Pool};
use r2d2::{self, ManageConnection, Config, PooledConnection as PConn};

#[derive(Debug)]
pub struct PooledConnection(PConn<ConnectionManager>);

lazy_static! {
    static ref POOL: RwLock<Option<Vec<r2d2::Pool<ConnectionManager>>>> = RwLock::new(None);
}

impl Client<(), ()>
{
    pub fn connection(self) -> Client<ConnectionOpts, Vec<Server>>
    {
        let opts = ConnectionOpts::default();
        Client {
            cmd: Command(opts, None),
            errors: self.errors,
        }
    }
}

#[derive(Debug)]
pub struct Server {
    host: &'static str,
    cfg: Config<Connection, Error>,
}

impl Server {
    pub fn new(arg: &'static str) -> Server {
        Server {
            host: arg,
            cfg: Config::default(),
        }
    }

    pub fn config(mut self, arg: Config<Connection, Error>) -> Server {
        self.cfg = arg;
        self
    }
}

impl Client<ConnectionOpts, Vec<Server>>
{
    pub fn servers(mut self, arg: Vec<Server>) -> Self {
        let s: Vec<_> = arg.iter().map(|s| s.host).collect();
        self.cmd.0.set_servers(s);
        self.cmd.1 = Some(arg);
        self
    }

    pub fn db(mut self, arg: &'static str) -> Self {
        self.cmd.0.set_db(arg);
        self
    }

    pub fn user(mut self, arg: &'static str) -> Self {
        self.cmd.0.set_user(arg);
        self
    }

    pub fn password(mut self, arg: &'static str) -> Self {
        self.cmd.0.set_password(arg);
        self
    }

    pub fn retries(mut self, arg: u8) -> Self {
        self.cmd.0.set_retries(arg);
        self
    }

    pub fn tls(mut self, arg: Option<TlsCfg>) -> Self {
        self.cmd.0.set_tls(arg);
        self
    }

    pub fn connect(self) -> Result<Pool> {
        set_config(self.cmd.0);
        let mut pools: Vec<r2d2::Pool<ConnectionManager>> = Vec::new();
        let servers = match self.cmd.1 {
            Some(servers) => servers,
            None => {
                let server = Server {
                    host: {
                        let opts = ConnectionOpts::default();
                        opts.servers()[0]
                    },
                    cfg: Config::default(),
                };
                vec![server]
            },
        };
        for server in servers {
            let manager = ConnectionManager(server.host);
            let pool = r2d2::Pool::new(server.cfg, manager)?;
            pools.push(pool);
        }
        set_pool(pools);
        Ok(Pool)
    }
}

pub struct ConnectionManager(&'static str);

impl ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        let opts = config().read();
        Connection::new(self.0, &opts)
    }

    fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
        conn.is_valid()
    }

    fn has_broken(&self, conn: &mut Connection) -> bool {
        if conn.broken() {
            return true;
        }
        match conn.stream().take_error() {
            Ok(error) => {
                if error.is_some() {
                    return true;
                }
            }
            Err(_) => {
                return true;
            }
        }
        false
    }
}

impl ReqlConnection for PooledConnection {
    fn stream(&mut self) -> &mut TcpStream {
        self.0.stream()
    }

    fn incr_token(&mut self) -> &mut Self {
        self.0.incr_token();
        self
    }

    fn token(&self) -> u64 {
        self.0.token()
    }

    fn set_broken(&mut self, b: bool) -> &mut Self {
        self.0.set_broken(b);
        self
    }

    fn broken(&self) -> bool {
        self.0.broken()
    }
}

impl Session for Pool {
    type Connection = PooledConnection;

    fn get(&self) -> Result<PooledConnection> {
        let cfg = config().read();
        let pool = pool().read();
        match *pool {
            Some(ref pool) => {
                let msg = String::from("Failed to get a connection.");
                let mut last_error = err!(ConnectionError::Other(msg));
                macro_rules! return_conn {
                    ($e:expr) => {{
                        match $e {
                            Ok(mut conn) => {
                                conn.incr_token();
                                return Ok(PooledConnection(conn));
                            },
                            Err(error) => last_error = err!(error),
                        }
                    }}
                }
                let mut num_retries = cfg.retries();
                while num_retries > 0 {
                    let mut least_connections = 0;
                    let mut least_connected_server = 0;
                    let mut most_idle = 0;
                    let mut most_idle_server = 0;
                    for (i, p) in pool.iter().enumerate() {
                        let state = p.state();
                        if least_connections == 0 || least_connections > state.connections {
                            least_connections = state.connections;
                            least_connected_server = i
                        }
                        if most_idle == 0 || most_idle < state.idle_connections {
                            most_idle = state.idle_connections;
                            most_idle_server = i
                        }
                    }
                    if most_idle > 0 {
                        return_conn!(pool[most_idle_server].get());
                    } else if least_connections > 0 {
                        return_conn!(pool[least_connected_server].get());
                    } else {
                        let msg = String::from("All servers are currently down.");
                        last_error = err!(ConnectionError::Other(msg));
                    }
                    num_retries -= 1;
                }
                return last_error;
            }
            None => {
                let msg = String::from("Your connection pool is not initialised. \
                                   Use `r.connection().connect()` to initialise the pool \
                                   before trying to send any connections to the database. \
                                   This is typically done in the `main` function.");
                return err!(ConnectionError::Other(msg));
            }
        }
    }
}

fn pool() -> &'static RwLock<Option<Vec<r2d2::Pool<ConnectionManager>>>> {
    &POOL
}

fn set_pool(p: Vec<r2d2::Pool<ConnectionManager>>) {
    let mut pool = POOL.write();
    *pool = Some(p);
}
