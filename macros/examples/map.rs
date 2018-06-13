#![feature(proc_macro, proc_macro_non_items)]

extern crate futures;
extern crate reql;
extern crate reql_macros;
#[macro_use]
extern crate serde_json;

use futures::Stream;
use reql_macros::args;
use reql::{Config, Client, Document, Run};

fn main() -> reql::Result<()> {
    // Create a new ReQL client
    let r = Client::new();

    // Create a connection pool
    let conn = r.connect(Config::default())?;

    // Run the query
    let sequence1 = json!([100, 200, 300, 400]);
    let sequence2 = json!([10, 20, 30, 40]);
    let sequence3 = json!([1, 2, 3, 4]);

    let sum = r.map(args!(sequence1, sequence2, sequence3, |val1, val2, val3| {
        val1.add(val2).add(val3)
    }))
    .run::<[i32; 4]>(conn)?;

    // Process results
    match sum.wait().next().unwrap()? {
        Some(Document::Expected(sum)) => {
            println!("{:?}", sum);
        }
        res => {
            println!("unexpected response from DB: {:?}", res);
        }
    }

    Ok(())
}
