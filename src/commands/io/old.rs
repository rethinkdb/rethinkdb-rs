extern crate ql2;
extern crate r2d2;
extern crate reql;
extern crate scram;
extern crate tokio_core;
extern crate byteorder;
extern crate futures;

mod handshake;

use std::{io, result, fmt};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr};

use reql::{Result, Connection as ReqlConnection};
use reql::errors::*;
use reql::commands::Client;

use futures::sync::oneshot;
use futures::{future, Future, Stream, Sink};
use tokio_core::reactor::{Handle, Remote};
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_core::net::TcpStream;
use byteorder::{LittleEndian, /* WriteBytesExt, ReadBytesExt, */ ByteOrder};
use scram::{ClientFirst, ServerFirst, ServerFinal};

#[derive(Debug, Clone)]
pub struct Pool(Vec<r2d2::Pool<Manager>>);

#[derive(Debug, Clone)]
pub struct Connection {
    //manager: Manager,
    broken: bool,
}

#[derive(Debug, Clone)]
pub struct Opts {
    addresses: Vec<SocketAddr>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    tls: Option<TlsCfg>,
}

#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: &'static str,
}

impl Default for Opts {
    fn default() -> Opts {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Opts {
            addresses: vec![SocketAddr::new(localhost, 28015)],
            db: "test",
            user: "admin",
            password: "",
            retries: 5,
            tls: None,
        }
    }
}

#[derive(Clone)]
struct Manager {
    opts: Opts,
    remote: Remote,
}

struct HandshakeCodec;
struct QueryCodec;

impl fmt::Debug for Manager {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.opts.fmt(formatter)
    }
}

pub type Config = r2d2::Config<Connection, Error>;

type RequestId = u64;

/// Create a new connection to the database server
pub trait Connect {
    type Connection: ReqlConnection;

    fn connect(&self, pairs: Vec<(Config, Opts)>, handle: &Handle) -> Result<Self::Connection>;
}

impl ReqlConnection for Connection {
    fn broken(&self) -> bool {
        self.broken
    }
}

impl ReqlConnection for Pool {
    fn broken(&self) -> bool {
        unimplemented!();
    }
}

impl Connect for Client {
    type Connection = Pool;

    fn connect(&self, pairs: Vec<(Config, Opts)>, handle: &Handle) -> Result<Pool> {
        let mut pairs = pairs;
        if pairs.is_empty() {
            let config = Config::default();
            let opts = Opts::default();
            pairs.push((config, opts));
        }
        let mut connection = Pool(Vec::new());
        for (config, opts) in pairs {
            let remote = handle.remote().clone();
            let manager = Manager {
                opts: opts,
                remote: remote,
            };
            match r2d2::Pool::new(config, manager) {
                Ok(pool) => { connection.0.push(pool); }
                Err(err) => {
                    let error = io::Error::new(io::ErrorKind::Other, err);
                    return Err(From::from(DriverError::Io(error)));
                }
            }
        }
        Ok(connection)
    }
}

impl Opts {
    /// Sets server
    pub fn set_server<T: ToSocketAddrs>(&mut self, server: T) -> Result<&mut Opts> {
        let mut addrs = Vec::new();
        for addr in server.to_socket_addrs()? {
            addrs.push(addr);
        }
        if addrs.is_empty() {
            let error = DriverError::Other("no addresses found".into());
            return Err(From::from(error));
        }
        self.addresses = addrs;
        Ok(self)
    }
    /// Sets database
    pub fn set_db(&mut self, db: &'static str) -> &mut Opts {
        self.db = db;
        self
    }
    /// Sets username
    pub fn set_user(&mut self, user: &'static str) -> &mut Opts {
        self.user = user;
        self
    }
    /// Sets password
    pub fn set_password(&mut self, password: &'static str) -> &mut Opts {
        self.password = password;
        self
    }
    /// Sets retries
    pub fn set_retries(&mut self, retries: u8) -> &mut Opts {
        self.retries = retries;
        self
    }
}

impl r2d2::ManageConnection for Manager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection> {
        Connection::new(self.clone())
    }

    //fn is_valid(&self, mut conn: &mut Connection) -> Result<()> {
    fn is_valid(&self, _: &mut Connection) -> Result<()> {
        unimplemented!();
    }

    fn has_broken(&self, conn: &mut Connection) -> bool {
        conn.broken()
    }
}

impl Connection {
    fn new(manager: Manager) -> Result<Connection> {
        let remote = manager.remote;
        let opts = manager.opts;
        let (tx, rx) = oneshot::channel();

        remote.spawn(move |handle| {
            for address in opts.addresses.iter() {
                if let Ok(stream) = TcpStream::connect(address, handle).wait() {
                    tx.complete(stream);
                    return Ok(());
                }
            }
            Err(())
        });

        let stream = match rx.wait() {
            Ok(stream) => stream,
            Err(err) => {
                let err = io::Error::new(io::ErrorKind::Other, err);
                return Err(From::from(err));
            }
        };

        let transport = stream.framed(HandshakeCodec);

        let mut version = [0; 4];
        LittleEndian::write_u32(&mut version, ql2::proto::VersionDummy_Version::V1_0 as u32);

        let handshake = transport
            // Send desired version to the server
            .send(version.as_ref().to_owned())
            
            // Send client first message
            .and_then(|transport| {
                let scram = try!(ClientFirst::new(opts.user, opts.password, None));
                let (scram, client_first) = scram.client_first();

                let ar = AuthRequest {
                    protocol_version: 0,
                    authentication_method: String::from("SCRAM-SHA-256"),
                    authentication: client_first,
                };
                let mut msg = try!(to_vec(&ar));
                msg.push(b'\0');

                transport.send(version.as_ref().to_owned())
            })

        .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
            .and_then(|(res, transport)| {
                match res {
                    Some(ref msg) => {
                        Ok(transport)
                    }
                    _ => {
                        let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
                        Err(err)
                    }
                }
            })
        ;

        Ok(Connection {
            //manager: manager,
            broken: false,
        })
    }
}

impl Codec for HandshakeCodec {
    type In = Vec<u8>;
    type Out = Vec<u8>;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Vec<u8>>> {
        match buf.as_slice().iter().position(|&b| b == b'\0') {
            Some(i) => {
                // Remove the serialized frame from the buffer
                let res = buf.drain_to(i)
                    .as_slice()
                    .to_owned();
                // Also remove the '\0'
                buf.drain_to(1);
                Ok(Some(res))
            }
            None => {
                // We don't yet have a full message
                Ok(None)
            }
        }
    }

    fn encode(&mut self, msg: Vec<u8>, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(&msg);
        Ok(())
    }
}

impl Codec for QueryCodec {
    type In = (RequestId, Vec<u8>);
    type Out = (RequestId, Vec<u8>);

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<(RequestId, Vec<u8>)>> {
        // At least 12 bytes are required for a frame
        // https://rethinkdb.com/docs/writing-drivers/#receive-responses
        if buf.len() < 12 {
            // We don't yet have a full message
            return Ok(None);
        }

        let id = LittleEndian::read_u64(buf.drain_to(8).as_slice());
        let size = LittleEndian::read_u32(buf.drain_to(4).as_slice()) as usize;
        if buf.len() < size {
            // We don't yet have a full message
            return Ok(None);
        }
        let res = buf.drain_to(size)
            .as_slice()
            .to_owned();
        Ok(Some((id, res)))
    }

    fn encode(&mut self, msg: (RequestId, Vec<u8>), buf: &mut Vec<u8>) -> io::Result<()> {
        let (id, msg) = msg;

        let mut encoded_id = [0; 8];
        LittleEndian::write_u64(&mut encoded_id, id as u64);

        let mut msg_len = [0; 4];
        LittleEndian::write_u32(&mut msg_len, msg.len() as u32);

        buf.extend(&encoded_id);
        buf.extend(&msg_len);
        buf.extend(&msg);

        Ok(())
    }
}

/*
   impl<T: Io + 'static> ClientProto<T> for Ql2Proto {
   type Request = Vec<u8>;
   type Response = Vec<u8>;

   type Transport = Framed<T, QueryCodec>;
   type BindTransport = io::Result<Self::Transport>;

   fn bind_transport(&self, io: T) -> Self::BindTransport {
   let transport = io.framed(QueryCodec);

   let mut version = [0; 4];
   LittleEndian::write_u32(&mut version, ql2::proto::VersionDummy_Version::V1_0 as u32);

   let handshake = transport.send(version.as_ref().to_owned())
   .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
   .and_then(|(res, transport)| {
   match res {
   Some(ref msg) => {
   Ok(transport)
   }
   _ => {
   let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
   Err(err)
   }
   }
   })
   ;

   Box::new(handshake)
   }
   }
   */
