use futures::executor::block_on;
use reql::r;

fn main() {
    let conn = block_on(r.connect(Default::default())).unwrap();
    let resp = block_on(r.expr("hello world").run(&conn, Default::default())).unwrap();
    println!("{:?}", resp);
}
