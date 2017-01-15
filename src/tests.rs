extern crate rspec;

use self::rspec::context::rdescribe;

use commands::*;
use r;

#[test]
fn reql_behaviour() {
    rdescribe("the commands", |ctx| {
        ctx.it("should allow chaining", || {
            let _ = r.db("heroes").table("marvel");
        });
    });
}
