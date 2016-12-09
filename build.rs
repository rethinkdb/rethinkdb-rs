extern crate skeptic;
extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/serde/opts.in.rs");
    let dst = Path::new(&out_dir).join("opts.rs");

    serde_codegen::expand(&src, &dst).unwrap();

    skeptic::generate_doc_tests(&["README.md"]);
}
