extern crate rspec;

use self::rspec::context::rdescribe;

use commands::*;

#[test]
fn reql_behaviour() {
    rdescribe("the commands", |ctx| {
        ctx.it("should allow chaining", || {
            let r = Command::new();
            let _ = r.db("heroes").table("marvel");
        });
    });
}
