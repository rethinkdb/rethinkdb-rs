use std::io;
use std::sync::Arc;

use reql_io::r2d2;
use errors::Error;
use super::io_error;
use {Client, ConnectionManager, Config, Opts, Connection, Result};
use std::net::TcpStream;
use reql_io::tokio_core::io::Io;
use reql_io::futures::{Future, Stream, Sink};

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        Connection::new()
    }

    fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
        Ok(())
        // conn.is_valid()
        /*
        conn.id += 1;
        let request = "[1,1]".as_bytes().to_owned();
        let response = "[1]".as_bytes().to_owned();
        conn.inner.clone().transport
            .send((conn.id, request))
            .and_then(|t| t.into_future().map_err(|(e, _)| e))
            .and_then(|(res, _)| {
                match res {
                    Some(ref msg) if msg == &(conn.id, response) => Ok(()),
                    _ => Err(io_error("invalid response")),
                }
            }).wait().map_err(|e| From::from(e))
        */
    }

    fn has_broken(&self, conn: &mut Connection) -> bool {
        conn.broken
    }
}

impl Connection {
    fn new() -> Result<Connection> {
        let cfg = Config::get();
        let logger = cfg.logger;
        //let remote = cfg.remote;
        let servers = cfg.cluster;
        debug!(logger, "cluster: {:?}", servers);

        for server in servers {
            let addresses = server.addresses.clone();
            for address in addresses {
                debug!(logger, "connecting to {}", address);
                match TcpStream::connect(&address) {
                    Ok(stream) => {
                        let logger = logger.new(o!(
                                "local_addr" => stream.local_addr()?.to_string(),
                                "peer_addr" => stream.peer_addr()?.to_string(),
                                ));
                        debug!(logger, "connected successfully");

                        let mut conn = Connection {
                            id: 0,
                            broken: false,
                            server: server,
                            stream: stream,
                            logger: logger,
                        };

                        //conn.handshake(cfg.opts)?;
                        return Ok(conn);
                    }
                    Err(error) => {
                        warn!(logger, "failed to connect to {}: {}", address, error);
                    }
                }
            }
        }

        Err(io_error("failed to connect to any server"))?
    }
}
