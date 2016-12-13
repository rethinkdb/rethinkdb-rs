#![allow(dead_code)]

use std::net::TcpStream;

use parking_lot::RwLock;
use super::{Client, Command};
use errors::*;
use conn::{
    ConnectionOpts,
    Connection,
    ReqlConnection,
    TlsCfg,
    Session,
};
use ::{Result, set_config, config, Pool};
use r2d2::{self, ManageConnection, Config, PooledConnection as PConn};

#[derive(Debug)]
pub struct PooledConnection(PConn<ConnectionManager>);

lazy_static! {
    static ref POOL: RwLock<Option<Vec<r2d2::Pool<ConnectionManager>>>> = RwLock::new(None);
}

impl Client<(), ()>
{
    pub fn connection(self) -> Client<(), ConnectionOpts>
    {
        let opts = ConnectionOpts::default();
        Client {
            cmd: Command((), Some(opts)),
            errors: self.errors,
        }
    }
}

macro_rules! set_opt {
    ($opts:expr, $func:ident($arg:ident)) => {
        match $opts.1 {
            Some(ref mut opts) => {
                opts.$func($arg);
            }
            None => {
                let mut opts = ConnectionOpts::default();
                opts.$func($arg);
                $opts.1 = Some(opts);
            }
        }
    }
}

impl Client<(), ConnectionOpts>
{
    pub fn servers(&mut self, servers: Vec<&'static str>) -> &mut Self {
        set_opt!(self.cmd, set_servers(servers));
        self
    }

    pub fn db(&mut self, db: &'static str) -> &mut Self {
        set_opt!(self.cmd, set_db(db));
        self
    }

    pub fn user(&mut self, user: &'static str) -> &mut Self {
        set_opt!(self.cmd, set_user(user));
        self
    }

    pub fn password(&mut self, password: &'static str) -> &mut Self {
        set_opt!(self.cmd, set_password(password));
        self
    }

    pub fn retries(&mut self, retries: u8) -> &mut Self {
        set_opt!(self.cmd, set_retries(retries));
        self
    }

    pub fn tls(&mut self, tls: Option<TlsCfg>) -> &mut Self {
        set_opt!(self.cmd, set_tls(tls));
        self
    }

    pub fn connect(&self) -> Result<Pool> {
        match self.cmd.1.clone() {
            Some(opts) => {
                set_config(opts.clone());
                let mut pools: Vec<r2d2::Pool<ConnectionManager>> = Vec::new();
                for server in opts.servers() {
                    let manager = ConnectionManager(server);
                    let config = Config::default();
                    let new_pool = r2d2::Pool::new(config, manager)?;
                    pools.push(new_pool);
                }
                set_pool(pools);
                Ok(Pool)
            },
            None => {
                let msg = String::from("ConnectionOpts is unset");
                return err!(DriverError::Other(msg));
            },
        }
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
