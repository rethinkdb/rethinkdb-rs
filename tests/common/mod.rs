use std::process::Command;

// This should stop any current running database, and start a new one
// that has the database 'test', containing the table 'test'
// and also has a user named 'bob', with password 'secret'
pub fn setup() {
    //setup code specific to your library's tests would go here
    Command::new("sh")
        .arg("-c")
        .arg("rm -rf tests/rethinkdb_data")
        .spawn()
        .expect("failed to kill databases");

    Command::new("sh")
        .arg("-c")
        .arg("cp -r tests/fresh_data tests/rethinkdb_data")
        .spawn()
        .expect("failed to kill databases");

    Command::new("sh")
        .arg("-c")
        .arg("killall rethinkdb")
        .spawn()
        .expect("failed to kill databases");

    std::thread::sleep(std::time::Duration::from_millis(500));

    Command::new("rethinkdb")
        .arg("--daemon")
        .arg("--directory")
        .arg("tests/rethinkdb_data")
        .spawn()
        .expect("failed to start database");
    // takes time for the test to get tables ready
    std::thread::sleep(std::time::Duration::from_secs(20));
}
