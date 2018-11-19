extern crate reql;
extern crate reql_types;
extern crate futures;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use reql::*;
use reql_types::*;
use futures::Stream;

use std::sync::*;
use std::io::*;

mod common;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scrap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub cmd: String,
    pub count: u32,
}

const CHANGES: u32 = 10000;

/// writes a large amount of data to server
fn writer() -> u32 {
    let conf = Config::default();

    let r = Client::new();

    let conn = r.connect(conf).unwrap();
    println!("Writer Connected");
    let mut scrap = Scrap {
        id: Some("hej".to_string()),
        cmd: "askldjhfasdfa;sldkjfal;skdjflfeuiahfiuseh faiusehf asiuehf aiuseh f".to_string(),
        count: 0,
    };
    let mut amount = 0;
    loop {
        //println!("sending {}", amount);
        scrap.count += 1;
        let stat = r.db("test")
            .table("tests")
            .get("hej")
            .replace(json!(scrap))
            .run::<WriteStatus>(conn);

        stat.unwrap().wait().next();
        amount += 1;

        if amount > CHANGES {
            break;
        }
    }
    amount
}



#[test]
/// Starts a thread that will write, a thread that reads the changes and then compares the amount
fn changefeeds_for_a_long_time() {
    common::setup();


    let counter = Arc::new(Mutex::new(0));

    let read_counter = Arc::clone(&counter);

    let _reader = std::thread::spawn(move || {
        let conf = Config::default();
        let r = Client::new();

        let conn = r.connect(conf).unwrap();
        println!("");
        println!("Reader Connected");

        let stat = r.db("test")
            .table("tests")
            .changes()
            .run::<Change<Scrap,Scrap>>(conn);

        let mut wait = stat.unwrap().wait();
        loop {
            let _data  = wait.next().unwrap();

            let mut amount = read_counter.lock().unwrap();
            *amount += 1;
            print!("Receiving {:6}/{} ... \r", *amount, CHANGES );
            let _ = stdout().flush();
        }
    });

    std::thread::sleep(std::time::Duration::from_millis(500));

    let writer = std::thread::spawn(|| {
        let val = writer();
        val
    });

    // wait until all is written
    let written = writer.join().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    assert_eq!(written, *counter.lock().unwrap(), "wrote and read diff amount");

}
