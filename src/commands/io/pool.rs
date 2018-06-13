use super::io_error;
use {Connection, Result, Session, SessionManager};
use errors::Error;
use r2d2;
use std::net::TcpStream;

impl r2d2::ManageConnection for SessionManager {
    type Connection = Session;
    type Error = Error;

    fn connect(&self) -> Result<Session> {
        Session::new(self.0)
    }

    fn is_valid(&self, conn: &mut Session) -> Result<()> {
        conn.is_valid()
    }

    fn has_broken(&self, conn: &mut Session) -> bool {
        conn.broken
    }
}

impl Session {
    fn new(conn: Connection) -> Result<Session> {
        let cfg = conn.config();
        let mut servers: Vec<_> = cfg.cluster.values().collect();
        servers.sort();
        debug!("cluster: {:?}", servers);

        for server in servers {
            for address in server.addresses.iter() {
                debug!("connecting to {}", address);
                match TcpStream::connect(&address) {
                    Ok(stream) => {
                        let mut conn = Session {
                            id: 0,
                            broken: false,
                            stream: stream,
                        };

                        conn.handshake(&cfg.opts)?;
                        debug!("connected successfully");
                        return Ok(conn);
                    }
                    Err(error) => {
                        warn!("failed to connect to {}: {}", address, error);
                        conn.set_latency()?;
                        if let Ok(session) = Self::new(conn) {
                            return Ok(session);
                        }
                    }
                }
            }
        }

        Err(io_error("failed to connect to any server"))?
    }
}
