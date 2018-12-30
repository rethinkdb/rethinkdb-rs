mod arg;

use crate::{
    cmd::{
        connect::Connection,
        run::{run, Opts},
    },
    r, Result,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Command(Bytes);

impl r {
    pub fn expr<A>(&self, arg: A) -> Command
    where
        A: Into<Arg>,
    {
        Command(arg.into().0)
    }
}

impl Command {
    pub async fn run<O, T>(self, conn: &Connection, opts: O) -> Result<T>
    where
        O: Into<Option<Opts>> + 'static,
        T: DeserializeOwned,
    {
        await!(run(conn, self.0, opts.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::r;
    use futures::executor::block_on;

    #[test]
    fn hello_world_works() -> crate::Result<()> {
        block_on(
            async {
                let conn = await!(r.connect(None))?;
                let resp: String = await!(r.expr("hello world").run(&conn, None))?;
                assert_eq!(resp, "hello world");
                Ok(())
            },
        )
    }
}
