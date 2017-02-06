use std::{io, result, fmt};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr};

use {Manager};

use reql::{Result, Connection as ReqlConnection};
use reql::errors::*;
use reql::commands::Command;

use futures::sync::oneshot;
use futures::{future, Future, Stream, Sink};
use tokio_core::reactor::{Handle, Remote};
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_core::net::TcpStream;
use byteorder::{LittleEndian, /* WriteBytesExt, ReadBytesExt, */ ByteOrder};
use scram::{ClientFirst, ServerFirst, ServerFinal};

struct HandshakeCodec;

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
