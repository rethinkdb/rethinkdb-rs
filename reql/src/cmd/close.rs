//! Close an open connection
//!
//! Closing a connection normally waits until all outstanding requests have
//! finished and then frees any open resources associated with the
//! connection. By passing `SkipNoreplyWait` as the argument, the connection
//! will be closed immediately, possibly aborting any outstanding noreply
//! writes.
//!
//! A noreply query is executed by passing the `noreply` option to the
//! [run](crate::Command::run) command, indicating that `run()` should not
//! wait for the query to complete before returning. You may also
//! explicitly wait for a noreply query to complete by using the
//! [noreply_wait](crate::Session::noreply_wait) command.
//!
//!
//! ## Example
//!
//! Close an open connection, waiting for noreply writes to finish.
//!
//! ```
//! # async fn example() -> reql::Result<()> {
//! # let session = reql::r.connect(()).await?;
//! # let mut conn = session.connection()?;
//! conn.close(()).await
//! # }
//! ```
//!
//! ## Example
//!
//! Close an open connection immediately.
//!
//! ```
//! # use reql::cmd::close::SkipNoreplyWait;
//! # async fn example() -> reql::Result<()> {
//! # let session = reql::r.connect(()).await?;
//! # let mut conn = session.connection()?;
//! conn.close(SkipNoreplyWait).await
//! # }
//! ```
//!
//! ## Related commands
//!
//! * [connect](crate::r::connect)
//! * [use](crate::Session::use_)

/// Skip waiting for `noreply` queries
#[derive(Debug)]
pub struct SkipNoreplyWait;

pub trait Arg {
    fn noreply_wait(self) -> bool;
}

impl Arg for () {
    fn noreply_wait(self) -> bool {
        true
    }
}

impl Arg for SkipNoreplyWait {
    fn noreply_wait(self) -> bool {
        false
    }
}
