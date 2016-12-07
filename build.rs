extern crate skeptic;
extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    for path in &["conn", "opts", "query"] {
        let src = format!("src/serde/{}.in.rs", path);
        let dst = format!("{}.rs", path);

        let src = Path::new(&src);
        let dst = Path::new(&out_dir).join(&dst);
        serde_codegen::expand(&src, &dst).unwrap();
    }

    skeptic::generate_doc_tests(&["README.md"]);
}
