use crate::Result;
use futures::prelude::*;
use romio::TcpStream;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Session<'a> {
    id: [u8; 8],
    stream: &'a TcpStream,
}

impl<'a> Session<'a> {
    pub(crate) fn new(id: u64, stream: &'a TcpStream) -> Self {
        Session {
            stream,
            id: id.to_le_bytes(),
        }
    }

    pub(crate) async fn write<'b>(mut self, buf: &'a [u8]) -> Result<Session<'a>> {
        await!(self.stream.write_all(&self.id))?;
        let buf_len = buf.len().to_le_bytes();
        await!(self.stream.write_all(&buf_len))?;
        await!(self.stream.write_all(buf))?;
        Ok(self)
    }
}
