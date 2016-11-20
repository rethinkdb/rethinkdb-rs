extern crate reql;
extern crate futures;
extern crate serde_json;

use reql::{r, Command};
use futures::stream::Stream;
use serde_json::Value;

fn main() {
    let db = "shows";
    let table = "posts";

    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db(db)
        .connect()
        .unwrap();

    // Create our database if necessary
    let res = r.db_create(db).run::<Value>().unwrap();
    for _ in res.wait() { }

    // Drop table if nessary
    let res = r.table_drop(table).run::<Value>().unwrap();
    for _ in res.wait() { }

    // Create our table
    let res = r.table_create(table).run::<Value>().unwrap();
    for _ in res.wait() { }

    // Insert user(s) into the table
    let posts = r#"
        [
            { "name": "William Adama", "tv_show": "Battlestar Galactica",
              "posts": [
                {"title": "Decommissioning speech", "content": "The Cylon War is long over..."},
                {"title": "We are at war", "content": "Moments ago, this ship received word..."},
                {"title": "The new Earth", "content": "The discoveries of the past few days..."}
              ]
            },
            { "name": "Laura Roslin", "tv_show": "Battlestar Galactica",
              "posts": [
                {"title": "The oath of office", "content": "I, Laura Roslin, ..."},
                {"title": "They look like us", "content": "The Cylons have the ability..."}
              ]
            },
            { "name": "Jean-Luc Picard", "tv_show": "Star Trek TNG",
              "posts": [
                {"title": "Civil rights", "content": "There are some words I've known since..."}
              ]
            }
        ]
    "#;
    let res = r.table(table).insert(posts).run::<Value>().unwrap();
    for _ in res.wait() { }
}
