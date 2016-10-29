extern crate reql;
extern crate rayon;
#[macro_use] extern crate slog;

use reql::r;
use reql::session::Client;
use rayon::prelude::*;

#[test]
fn connection_pool_works() {
    let logger = Client::logger().read();
    // Setup the connection
    r.connection()
        .set_servers(vec!["localhost:28015", "localhost:28016", "localhost:28017"])
        .set_db("blog")
        .connect()
        .unwrap();

    // Try arbitrary expressions
    r.expr(200).run().unwrap();

    // Create our database if necessary
    r.db_create("blog").run().unwrap();

    // Drop table if nessary
    r.table_drop("users").run().unwrap();

    // Create our table if necessary
    r.table_create("users").run().unwrap();

    // Insert 1 user(s) into the table
    (0..1u32)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, _)| {
            /*
            let user = r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build();
            */
            let user = r#"
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
            let res = r.table("users").insert(user).run();
            debug!(logger, "{:?}", res);
        });
}
