use futures::executor::block_on;
use reql::Client;

fn main() {
    let mut r = Client::new();
    let conn = block_on(r.connect()).unwrap();
    println!("{:?}", conn);
}
