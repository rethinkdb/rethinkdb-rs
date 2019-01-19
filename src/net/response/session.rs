use std::str::from_utf8;

use crate::{
    net::connection::{Connection, RequestId},
    Result,
};
use bytes::{Buf, BufMut, BytesMut, IntoBuf};
use futures::{channel::oneshot::Canceled, prelude::*};

const HEADER_LEN: usize = 8 + 4;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Session<'a> {
    id: RequestId,
    conn: &'a Connection,
}

impl<'a> Session<'a> {
    pub(crate) fn new(id: RequestId, conn: &'a Connection) -> Self {
        Session { id, conn }
    }

    pub(crate) async fn write(self, data: &'a [u8]) -> Result<()> {
        let data_len = data.len();
        let mut buf = BytesMut::with_capacity(HEADER_LEN + data_len);
        buf.put_u64_le(self.id as u64);
        buf.put_u32_le(data_len as u32);
        buf.put(data);
        log::debug!(
            "id => {}; sending query; data => {}",
            self.id,
            from_utf8(data).unwrap()
        );
        let mut stream = self.conn.stream();
        await!(stream.write_all(&buf))?;
        log::debug!("id => {}; query sent", self.id);
        Ok(())
    }

    pub(crate) fn read(self) -> impl Future<Output = Result<()>> + 'a {
        async move {
            let mut buf = BytesMut::new();
            buf.resize(HEADER_LEN, 0);
            let mut reader = self.conn.stream();
            let mut guard = await!(self.conn.senders().lock());
            log::debug!("id => {}; peeking socket data", self.id);
            await!(reader.read_exact(&mut buf))?;
            let mut header = buf.take().into_buf();
            let id = header.get_u64_le() as usize;
            log::debug!(
                "id => {}; peeked successfully, got data for {}",
                self.id,
                id
            );
            let len = header.get_u32_le() as usize;
            buf.resize(len, 0);
            log::debug!("id => {}; retrieving data", self.id);
            await!(reader.read_exact(&mut buf))?;
            let resp = buf.freeze();
            log::debug!(
                "id => {}; data retrieved; data => {}",
                self.id,
                from_utf8(&resp).unwrap()
            );
            let sender = guard.remove(id);
            sender.send(resp).map_err(|_| Canceled)?;
            Ok(())
        }
    }
}
