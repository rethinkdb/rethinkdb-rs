mod arg;

use crate::{
    cmd::{
        connect::Connection,
        merge::{self, merge, Merge},
        run::{run, Opts},
    },
    r, Result,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Expr(Bytes);

impl r {
    /// Construct a ReQL JSON object from a native object
    ///
    /// **Example:** Objects wrapped with `expr` can then be manipulated by ReQL API functions.
    ///
    /// ```rust
    /// # use reql::r;
    /// # use serde_json::json;
    /// #
    /// r.expr(json!({"a": "b"})).merge(json!({"b": [1, 2, 3]}))
    /// # ;
    /// ```
    pub fn expr<A>(&self, arg: A) -> Expr
    where
        A: Into<Arg>,
    {
        Expr(arg.into().0)
    }
}

impl Expr {
    pub fn merge<A>(&self, arg: A) -> Merge
    where
        A: Into<merge::Arg>,
    {
        merge(&self.0, arg)
    }

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
