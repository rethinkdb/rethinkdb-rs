use r2d2;
use errors::Error;
use super::io_error;
use {SessionManager, Connection, Session, Result};
use std::net::TcpStream;

impl r2d2::ManageConnection for SessionManager {
    type Connection = Session;
    type Error = Error;

    fn connect(&self) -> Result<Session> {
        Session::new(self.0)
    }

    fn is_valid(&self, mut conn: &mut Session) -> Result<()> {
        conn.is_valid()
    }

    fn has_broken(&self, conn: &mut Session) -> bool {
        conn.broken
    }
}

impl Session {
    fn new(conn: Connection) -> Result<Session> {
        let cfg = conn.config();
        let logger = cfg.logger;
        //let remote = cfg.remote;
        let servers = cfg.cluster;
        debug!(logger, "cluster: {:?}", servers);

        for server in servers {
            for address in server.addresses {
                debug!(logger, "connecting to {}", address);
                match TcpStream::connect(&address) {
                    Ok(stream) => {
                        let logger = logger.new(o!(
                            "local_addr" => stream.local_addr()?.to_string(),
                            "peer_addr" => address.to_string(),
                        ));

                        let mut conn = Session {
                            id: 0,
                            broken: false,
                            stream: stream,
                            logger: logger,
                        };

                        conn.handshake(&cfg.opts)?;
                        debug!(conn.logger, "connected successfully");
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
