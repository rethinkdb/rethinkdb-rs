extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate serde_derive;

mod config;

use config::Config;

fn main() {
    let cfg = Config::new();
    for section in cfg.menu {
        for command in section.commands {
            let path = format!("{}/{}/{}.md", cfg.docs_dir.display(), section.name, command.permalink);
            println!("cargo:rerun-if-changed={}", path);
        }
    }
}
