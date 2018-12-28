mod arg;

use crate::{
    r, Result,
    cmd::{
        run::{run, Opts},
        connect::Connection,
    },
};
use bytes::Bytes;
use serde::de::DeserializeOwned;

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Command(Bytes);

impl r {
    pub fn expr(&self, arg: impl Into<Arg>) -> Command {
        Command(arg.into().0)
    }
}

impl Command {
    pub async fn run<O, T>(self, conn: &Connection, opts: O) -> Result<T>
        where O: Into<Option<Opts>> + 'static,
              T: DeserializeOwned 
    {
        await!(run(conn, self.0, opts.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::r;
    use futures::executor::block_on;

    #[test]
    fn hello_world_works() {
        let conn = block_on(r.connect(None)).unwrap();
        let resp = r.expr("hello world").run(&conn, None);
        let _: String = block_on(resp).unwrap();
    }
}
