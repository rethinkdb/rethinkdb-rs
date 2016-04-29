//! RethinkDB Connection

use ql2::proto;
use std::net::TcpStream;
use std::io::Write;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use bufstream::BufStream;
use std::io::BufRead;

/// Options
#[derive(Debug)]
pub struct Opts {
    pub host: &'static str,
    pub port: u16,
    pub db: &'static str,
    pub user: &'static str,
    pub password: &'static str,
    pub timeout: u16,
    pub ssl: Option<String>,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts {
            host: "localhost",
            port: 28015,
            db: "test",
            user: "admin",
            password: "",
            timeout: 20,
            ssl: None,
        }
    }
}

/// A connection to a RethinkDB database.
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String,
    token    : usize,
}

impl Connection {
    pub fn new(opts: Opts) -> Connection {
        let stream = TcpStream::connect((opts.host, opts.port)).ok().unwrap();

        let mut conn = Connection{
            host    : opts.host.to_string(),
            port    : opts.port,
            stream  : stream,
            auth    : "AUTH".to_string(),
            token   : 1,
        };

        conn.handshake();
        conn
    }

    fn handshake(&mut self)  {
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Version::V0_4 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(proto::VersionDummy_Protocol::JSON as u32);
        self.stream.flush();

        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
        }

    }
}
