extern crate reql;

use reql::{r, Run};
//use reql::types;
/*
use ql2::commands::ReadMode::Outdated;
use ql2::commands::IdentifierFormat::Uuid;
*/
#[test]
fn db_works() {
    //r.connection().connect().unwrap();
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

    let query = r.table("heroes").run();
    panic!(format!("{:?}", query));
}
