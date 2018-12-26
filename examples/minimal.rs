use futures::executor::block_on;
use reql::Client;

fn main() {
    let r = Client::default();
    let conn = block_on(r.connect()).unwrap();
    println!("{:?}", conn);
}
