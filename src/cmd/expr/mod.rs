mod arg;

use crate::r;
use bytes::Bytes;

pub use self::arg::Arg;

#[derive(Debug, Clone)]
pub struct Expr {
    pub(super) bytes: Bytes,
}

impl r {
    /// Construct a ReQL JSON object from a native object
    ///
    /// ## Example
    ///
    /// Objects wrapped with `expr` can then be manipulated by ReQL API functions.
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
        Expr {
            bytes: arg.into().bytes,
        }
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
                let conn = await!(r.connect(()))?;
                let resp: String = await!(r.expr("hello world").run(&conn))?;
                assert_eq!(resp, "hello world");
                Ok(())
            },
        )
    }
}
