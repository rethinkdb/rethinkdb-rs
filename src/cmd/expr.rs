use crate::{
    r, Connection, Result, opt, error,
    cmd::{
        Expr,
        run::Session,
    },
};

const _TYPE: u32 = 1;

impl r {
    pub fn expr(&self, _arg: &str) -> Expr {
        Expr
    }
}

impl Expr {
    pub async fn run(self, conn: &Connection, _opt: opt::Run) -> Result<()> {
        if conn.broken() {
            return Err(error::Driver::ConnectionBroken)?;
        }
        let id = conn.token()?;
        let sess = Session::new(id, conn.stream());
        await!(sess.write(br#"[1, "hello world", {}]"#))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::r;
    use futures::executor::block_on;

    #[test]
    fn hello_world_works() {
        let conn = block_on(r.connect(Default::default())).unwrap();
        let resp = r.expr("hello world").run(&conn, Default::default());
        block_on(resp).unwrap();
    }
}
