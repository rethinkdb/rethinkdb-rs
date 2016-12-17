extern crate reql;

use reql::{r, Run};

fn main() {
    r.connection()
        .db("rethinkdb")
        .connect()
        .unwrap();

    let servers = r.table("server_config").run::<()>();

    for server in servers {
        println!("{:?}", server);
    }
}
