extern crate reql;

use reql::r;
use reql::prelude::*;

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
    let res: Response<Value> = r.db_create(db).run().unwrap();
    for _ in res.wait() { }

    // Drop table if nessary
    let res: Response<Value> = r.table_drop(table).run().unwrap();
    for _ in res.wait() { }

    // Create our table
    let res: Response<Value> = r.table_create(table).run().unwrap();
    for _ in res.wait() { }

    // Insert user(s) into the table
    let posts: Value = from_str(r#"
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
    "#).unwrap();
    let res: Response<Value> = r.table(table).insert(posts).run().unwrap();
    for _ in res.wait() { }
}
