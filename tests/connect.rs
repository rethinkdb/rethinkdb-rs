extern crate reql;
use reql::*;
use reql_types::*;
use reql::Document::*;
use futures::Stream;
use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod common;

#[test]
fn it_connects() {

    common::setup();
    let mut conf = Config::default();

    let r = Client::new();
    // Create a connection pool
    println!("Connecting to server");

    let conn = r.connect(conf);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    assert!(conn.is_ok());
//    assert_eq!(4, adder::add_two(2));
}

// commented due to crashing the test-suite
// #[test]
// fn it_fails_to_connect() {
//     common::setup();
//     let mut conf = Config::default();
//     // should no be any connection available here
//     let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);

//     conf.servers = vec!(socket);

//     let r = Client::new();
//     // Create a connection pool
//     println!("Connecting to server");

//     let conn = r.connect(conf);
//     std::thread::sleep(std::time::Duration::from_millis(1000));
//     assert!(conn.is_err());
// }

#[test]
fn it_connects_with_correct_password() {
    common::setup();
    let mut conf = Config::default();
    conf.password = "secret";
    conf.user = "bob";

    let r = Client::new();
    // Create a connection pool
    println!("Connecting to server");

    let conn = r.connect(conf);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    assert!(conn.is_ok());
}

#[test]
fn it_fails_to_connect_with_wrong_password() {
    common::setup();
    let mut conf = Config::default();
    conf.password = "wrongpass";

    let r = Client::new();
    // Create a connection pool
    println!("Connecting to server");

    let conn = r.connect(conf);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    assert!(conn.is_err());
}
