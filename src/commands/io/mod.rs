mod pool;
//mod handshake;

use std::{io, error};
use std::net::ToSocketAddrs;

use {Client, Config, ConnectionManager, Server, Result, Pool, Opts, Response, ToArg};
use ql2::proto::{Term, Datum};
use reql_io::r2d2;
use reql_io::parking_lot::RwLock;
use reql_io::tokio_core::reactor::Remote;
use slog::Logger;

lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::new(None);
}

pub fn connect<A: ToArg>(client: &Client, args: A) -> Result<Pool> {
    let arg = args.to_arg();
    let logger = client.logger.new(o!("command" => "connect"));
    let query = format!("{}.connect({})", client.query, arg.string);
    debug!(logger, "{}", query);
    Config::init(arg.term, arg.remote, logger.clone())?;
    info!(logger, "creating connection pool...");
    let config = r2d2::Config::default();
    let r2d2 = r2d2::Pool::new(config, ConnectionManager).map_err(|err| io_error(err))?;
    info!(logger, "connection pool created successfully");
    Ok(Pool(r2d2))
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

impl Config {
    fn init(mut term: Term, remote: Option<Remote>, logger: Logger) -> Result<()> {
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
            hosts.push("localhost:28015".into());
        }

        for host in hosts {
            let addresses = host.to_socket_addrs()?;
            cluster.push(Server {
                addresses: addresses.collect(),
                latency: 0,
            });
        }

        let mut cfg = CONFIG.write();
        *cfg = Some(Config {
            cluster: cluster,
            opts: opts,
            remote: remote,
            logger: logger,
        });

        Ok(())
    }

    pub fn get() -> Config {
        if let Some(ref cfg) = *CONFIG.read() {
            return cfg.clone();
        }
        panic!("Config must be initialised before calling Config::get()");
    }
}
