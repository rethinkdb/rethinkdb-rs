extern crate reql;
extern crate reql_types;
extern crate futures;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use reql::*;
use reql_types::*;
use reql::Document::*;
use futures::Stream;

mod common;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Scrap {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    cmd: String,
    count: u32,
}

#[test]
/// writes a custom doc into the database and verifies the server returned inserted ok
fn can_write() {
    common::setup();

    let conf = Config::default();

    let r = Client::new();
    // Create a connection pool
    println!("Connecting to server");

    let conn = r.connect(conf).unwrap();

    let scrap = Scrap {
        id: Some("hej".to_string()),
        cmd: "askldjhfasdfa;sldkjfal;skdjflfeuiahfiuseh faiusehf asiuehf aiuseh f".to_string(),
        count: 0,
    };

    let stat = r.db("test")
        .table("tests")
        .insert(json!(scrap))
        .run::<WriteStatus>(conn);

    let data = stat.unwrap().wait().next();

    if let Some(Ok(Some(Expected(result)))) = data {
        if result.inserted == 1 {
            assert!(true);
        }
    } else {
        assert!(false);
    }

}

/// Reads the server status fields and verifies that the correct port has been inserted in the field
#[test]
fn can_read() {
    common::setup();

    let conf = Config::default();

    let r = Client::new();
    // Create a connection pool
    println!("Connecting to server");

    let conn = r.connect(conf).unwrap();

    let stat = r.db("rethinkdb")
        .table("server_status")
        .run::<ServerStatus>(conn);

    let data = stat.unwrap().wait().next();

    if let Some(Ok(Some(Expected(result)))) = data {
        if result.network.reql_port == 28015 {
            assert!(true);
        }
    } else {
        assert!(false);
    }

}
