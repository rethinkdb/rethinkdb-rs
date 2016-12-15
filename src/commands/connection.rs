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
    pub fn connection(self) -> Client<Config<Connection, Error>, ConnectionOpts>
    {
        let opts = ConnectionOpts::default();
        let cfg = Config::default();
        Client {
            cmd: Command(cfg, Some(opts)),
            errors: self.errors,
        }
    }
}

impl Client<Config<Connection, Error>, ConnectionOpts>
{
    pub fn pool_config(mut self, arg: Config<Connection, Error>) -> Self {
        self.cmd.0 = arg;
        self
    }

    pub fn servers(mut self, arg: Vec<&'static str>) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_servers(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn db(mut self, arg: &'static str) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_db(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn user(mut self, arg: &'static str) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_user(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn password(mut self, arg: &'static str) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_password(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn retries(mut self, arg: u8) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_retries(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn tls(mut self, arg: Option<TlsCfg>) -> Self {
        let mut opts = self.cmd.opts();
        opts.set_tls(arg);
        self.cmd.1 = Some(opts);
        self
    }

    pub fn connect(self) -> Result<Pool> {
        let opts = self.cmd.opts();
        set_config(opts.clone());
        let mut pools: Vec<r2d2::Pool<ConnectionManager>> = Vec::new();
        for server in opts.servers() {
            let manager = ConnectionManager(server);
            let config = self.clone_pool_config();
            let new_pool = r2d2::Pool::new(config, manager)?;
            pools.push(new_pool);
        }
        set_pool(pools);
        Ok(Pool)
    }

    fn clone_pool_config(&self) -> Config<Connection, Error>
    {
        let ref cfg = self.cmd.0;
        Config::builder()
            .pool_size(cfg.pool_size())
            .min_idle(cfg.min_idle())
            .helper_threads(cfg.helper_threads())
            .test_on_check_out(cfg.test_on_check_out())
            .initialization_fail_fast(cfg.initialization_fail_fast())
            .max_lifetime(cfg.max_lifetime())
            .idle_timeout(cfg.idle_timeout())
            .connection_timeout(cfg.connection_timeout())
            .build()
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
