use {
    crate::{
        cmd::run::{self, Run},
        err,
        net::{
            connection::{Connection, RequestId},
            response::{
                message::{Message, SuccessType},
                Response,
            },
        },
        Result,
    },
    bytes::{Buf, BufMut, Bytes, BytesMut, IntoBuf},
    futures::{channel::mpsc, prelude::*, ready, try_poll, try_ready, Poll},
    serde::de::DeserializeOwned,
    std::{pin::Pin, str::from_utf8, task::LocalWaker},
};

const HEADER_LEN: usize = 8 + 4;

#[derive(Debug, Clone)]
pub(crate) struct Session<'a> {
    id: RequestId,
    conn: &'a Connection,
}

struct Write<'a, 'b> {
    session: &'a Session<'b>,
    data: &'b [u8],
}

struct Read<'a, 'b>(&'a Session<'b>);

impl<'a> Session<'a> {
    fn new(id: RequestId, conn: &'a Connection) -> Self {
        Session { id, conn }
    }
}

// TODO this implementation is currently horribly inefficient as it polls continuously
// rather than waiting for notifications. Unfortunately, some external futures
// are not currently waking up so the entire thing ends up hanging indefinitely.
impl<'a, T> Stream for Run<'a, T>
where
    T: DeserializeOwned + Unpin,
{
    type Item = Result<Response<T>>;

    fn poll_next(mut self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Option<Self::Item>> {
        use {run::State::*, Poll::*, SuccessType::*};
        match self.state {
            New => {
                // We can't use `crate::ser::to_vec` here because it will wrap
                // the DB term in an array. Luckily, the options to `run` do not
                // contain arrays so we can safely use the upstream `to_vec`
                // here.
                let opts = match serde_json::to_vec(&self.opts) {
                    Ok(opts) => opts,
                    Err(error) => {
                        self.state = Done;
                        lw.wake();
                        return Ready(Some(Err(error.into())));
                    }
                };
                let opts_len = opts.len();
                let (header, sep, footer) = ("[1,", ",", "]");
                let len = header.len() + self.query.len() + sep.len() + opts_len + footer.len();
                let mut msg = BytesMut::with_capacity(len);
                msg.put(header);
                msg.put(&self.query);
                // don't include an empty object
                if opts_len > 2 {
                    msg.put(sep);
                    msg.put(opts);
                }
                msg.put(footer);
                self.query = msg.freeze();
                self.state = Initialised;
                lw.wake();
                return Pending;
            }
            Initialised => {
                let (sender, receiver) = mpsc::unbounded();
                self.receiver = Some(receiver);
                let mut senders = match Pin::new(&mut self.conn.senders().lock()).poll(lw) {
                    Ready(senders) => senders,
                    Pending => {
                        lw.wake();
                        return Pending;
                    }
                };
                let id = senders.insert(sender);
                let session = Session::new(id, self.conn);
                self.session = Some(session);
                self.state = SessionCreated;
                lw.wake();
                return Pending;
            }
            SessionCreated => {
                let poll = {
                    let session = self.session.as_ref().unwrap();
                    let data = &self.query;
                    let mut future = Write { session, data };
                    Pin::new(&mut future).poll(lw)
                };
                lw.wake();
                return match poll {
                    Ready(Ok(..)) => {
                        self.state = SessionWritten;
                        Pending
                    }
                    Ready(Err(error)) => {
                        self.state = Done;
                        Ready(Some(Err(error.into())))
                    }
                    Pending => Pending,
                };
            }
            SessionWritten => {
                let poll = {
                    let session = self.session.as_ref().unwrap();
                    let mut future = Read(session);
                    Pin::new(&mut future).poll(lw)
                };
                lw.wake();
                return match poll {
                    Ready(Ok(..)) => {
                        self.state = SessionRead;
                        Pending
                    }
                    Ready(Err(error)) => {
                        self.state = Done;
                        Ready(Some(Err(error.into())))
                    }
                    Pending => Pending,
                };
            }
            SessionRead => {
                let receiver = self.receiver.as_mut().unwrap();
                let resp = match Pin::new(&mut receiver.next()).poll(lw) {
                    Ready(Some(resp)) => resp,
                    Ready(None) => {
                        self.state = Done;
                        return Ready(None);
                    }
                    Pending => {
                        lw.wake();
                        return Pending;
                    }
                };
                let msg: Message<_> = match serde_json::from_slice(&resp) {
                    Ok(msg) => msg,
                    Err(error) => {
                        let error = match from_utf8(&resp) {
                            Ok(response) => err::Driver::Other(format!(
                                "failed to parse database response: {}; {}",
                                response, error
                            ))
                            .into(),
                            Err(..) => error.into(),
                        };
                        self.state = Done;
                        lw.wake();
                        return Ready(Some(Err(error)));
                    }
                };
                let (t, r, p) = match msg.extract() {
                    Ok(msg) => msg,
                    Err(error) => {
                        self.state = Done;
                        lw.wake();
                        return Ready(Some(Err(error.into())));
                    }
                };
                lw.wake();
                return match t {
                    SuccessAtom | SuccessSequence | ServerInfo => {
                        self.state = Done;
                        Ready(Some(Ok(Response::new(r, p))))
                    }
                    SuccessPartial => {
                        self.query = Bytes::from_static(b"[2]");
                        self.state = SessionCreated;
                        Ready(Some(Ok(Response::new(r, p))))
                    }
                    WaitComplete => {
                        self.state = Done;
                        Ready(None)
                    }
                };
            }
            Done => {
                return Ready(None);
            }
        }
    }
}

impl<'a, 'b> Future for Write<'a, 'b> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        let Write { session, data } = *self;
        let data_len = data.len();
        let mut buf = BytesMut::with_capacity(HEADER_LEN + data_len);
        buf.put_u64_le(session.id as u64);
        buf.put_u32_le(data_len as u32);
        buf.put(data);
        log::debug!(
            "id => {}; sending query; data => {}",
            session.id,
            from_utf8(data).unwrap()
        );
        let mut stream = session.conn.stream();
        try_ready!(Pin::new(&mut stream.write_all(&buf)).poll(lw));
        log::debug!("id => {}; query sent", session.id);
        Poll::Ready(Ok(()))
    }
}

impl<'a, 'b> Future for Read<'a, 'b> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        let Read(session) = *self;
        let mut buf = BytesMut::new();
        buf.resize(HEADER_LEN, 0);
        let mut reader = session.conn.stream();
        let senders = ready!(Pin::new(&mut session.conn.senders().lock()).poll(lw));
        log::debug!("id => {}; retrieving header information", session.id);
        try_ready!(Pin::new(&mut reader.read_exact(&mut buf)).poll(lw));
        let mut header = buf.take().into_buf();
        let id = header.get_u64_le() as usize;
        log::debug!(
            "id => {}; header retrieved, got data for {}",
            session.id,
            id
        );
        let len = header.get_u32_le() as usize;
        buf.resize(len, 0);
        loop {
            log::debug!("id => {}; retrieving data", session.id);
            let poll = Pin::new(&mut reader.read_exact(&mut buf)).poll(lw);
            if let Poll::Ready(..) = try_poll!(poll) {
                let resp = buf.freeze();
                log::debug!(
                    "id => {}; data retrieved; data => {}",
                    session.id,
                    from_utf8(&resp).unwrap()
                );
                let sender = senders.get(id).unwrap();
                return match sender.unbounded_send(resp) {
                    Ok(..) => Poll::Ready(Ok(())),
                    Err(e) => Poll::Ready(Err(e.into_send_error().into())),
                };
            }
        }
    }
}

impl<'a, T> Future for Run<'a, T>
where
    T: DeserializeOwned + Unpin,
{
    type Output = Result<Response<T>>;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        let resp = ready!(self.poll_next(lw));
        let result = resp.expect("can't convert a consumed Stream to a Future");
        Poll::Ready(result)
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        loop {
            if let Some(mut guard) = self.conn.senders().try_lock() {
                guard.remove(self.id);
                break;
            }
        }
    }
}
