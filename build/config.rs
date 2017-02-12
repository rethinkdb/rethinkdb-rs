use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use serde_yaml as yaml;

#[derive(Debug, Clone)]
pub struct Config {
    pub docs_dir: PathBuf,
    pub cmds_src: PathBuf,
    pub version: String,
    pub menu: Vec<Section>,
}

#[derive(Debug, Clone, Deserialize)]
struct Menu {
    section: Section,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Section {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub name: String,
    pub permalink: String,
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
            menu: build_menu(&menu_file).into_iter()
                .map(|menu| menu.section)
                .map(|mut section| {
                    if section.name == "Accessing ReQL" {
                        section.name = String::from("Accessing RQL");
                    }
                    section.name = section.name
                        .replace(" ", "-")
                        .to_lowercase();
                    section
                })
            .collect(),
        }
    }
}

fn build_menu(menu_file: &str) -> Vec<Menu> {
    let file = File::open(&menu_file).unwrap();
    let buf = BufReader::new(file);
    yaml::from_reader(buf).unwrap()
}
