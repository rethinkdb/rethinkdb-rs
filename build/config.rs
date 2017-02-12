use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use yaml_rust::{YamlLoader, Yaml};

#[derive(Debug, Clone)]
pub struct Config {
    pub docs_dir: PathBuf,
    pub cmds_src: PathBuf,
    pub version: String,
    pub menu: Vec<Yaml>,
}

impl Config {
    pub fn new() -> Config {
        let lang = "java";

        let out_dir = env::var("OUT_DIR").unwrap();
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let docs_dir = format!("{}/build/docs/api/{}", root_dir, lang);
        let menu_file = format!("{}/build/docs/_jekyll/_data/api_{}.yml", root_dir, lang);
        let cmds_src = format!("{}/commands.rs", out_dir);
        let version = env::var("CARGO_PKG_VERSION").unwrap();

        Config {
            docs_dir: PathBuf::from(&docs_dir),
            cmds_src: PathBuf::from(&cmds_src),
            version: version,
            menu: build_menu(&menu_file),
        }
    }
}

fn build_menu(menu_file: &str) -> Vec<Yaml> {
    let file = File::open(&menu_file).unwrap();
    let mut buf = BufReader::new(file);
    let mut yaml = String::new();
    buf.read_to_string(&mut yaml).unwrap();
    YamlLoader::load_from_str(&yaml).unwrap()
}
