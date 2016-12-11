extern crate reql;
#[macro_use]
extern crate slog;
extern crate slog_term;

use reql::{r, Run};
use slog::{Logger, DrainExt};
//use reql::types;
/*
use ql2::commands::ReadMode::Outdated;
use ql2::commands::IdentifierFormat::Uuid;
*/
#[test]
fn db_works() {
    let log = Logger::root(slog_term::streamer().build().fuse(), o!("test" => "driver"));
    reql::set_logger(&log);
    r.connection().connect().unwrap();
    /*
    let query = r.table("heroes").map(|hero: Arg| {
            //(row as Command<types::Object, ()>).get_field::<_, types::Object>("villain")
            hero.get_field::<_, types::Object>("villain")
        })
        ;
        */
    /*
    let query = r.table("heroes").map(|hero: Arg| {
            //(row as Command<types::Object, ()>).get_field::<_, types::Object>("villain")
            hero.get_field::<_, types::Object>("villain")
        })
        ;
    */

    /*
    let query = r.map("John Doe", |seq| {
        //seq.info()
    });
    */

    let query = r.table("heroes").run::<()>();
    for res in query {
        debug!(log, "{:?}", res);
    }
}
