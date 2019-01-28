mod arg;

use crate::{r, Client};

pub use arg::Arg;

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
    pub fn expr<A>(&self, arg: A) -> Client
    where
        A: Into<Arg>,
    {
        Client(arg.into().arg)
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
                let resp = await!(r.expr("hello world").run(&conn))?;
                assert_eq!(resp.first(), Some(&"hello world".to_owned()));
                Ok(())
            },
        )
    }
}
