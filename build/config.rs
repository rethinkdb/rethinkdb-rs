use serde_yaml as yaml;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub docs_dir: PathBuf,
    pub cmds_src: PathBuf,
    pub version: String,
    pub menu: Vec<Command>,
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
    #[serde(default)]
    pub section: String,
    pub permalink: String,
    pub method: Option<String>,
    pub typ: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let lang = "javascript";

        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let docs_dir = format!("{}/build/docs/api/{}", root_dir, lang);
        let menu_file = format!("{}/build/docs/_jekyll/_data/api_{}.yml", root_dir, lang);
        let out_dir = env::var("OUT_DIR").unwrap();
        let cmds_src = format!("{}/commands.rs", out_dir);
        let version = env::var("CARGO_PKG_VERSION").unwrap();

        let menu = build_menu(&menu_file).into_iter()
            // We are only interested in actual commands
            // that are contained in the sections
            .map(|menu| menu.section)
            // Rename sections
            .map(|mut section| {
                let sections = vec![
                    ("Geospatial commands", "Geospatial"),
                    ("Accessing ReQL", "Accessing RQL"),
                ];
                for (old, new) in sections {
                    if section.name == old {
                        section.name = new.to_owned();
                    }
                }
                section
            })
            // Match filesystem formating of section directories
            .map(|mut section| {
                section.name = section.name
                    .replace(" ", "-")
                    .to_lowercase();
                section
            });

        let mut commands = Vec::new();
        for section in menu {
            for mut command in section.commands {
                command.section = section.name.clone();
                commands.push(command);
            }
        }
        commands = commands.into_iter()
            // Drop blacklisted sections
            .filter(|command| {
                let blacklist = vec!["cursors"];
                for section in blacklist {
                    if section == command.section {
                        return false;
                    }
                }
                true
            })
        // Drop blacklisted commands
        .filter(|command| {
            let blacklist = vec![
                "r", "args", "use", "row", "opt_arg", "array", "object", "close", "reconnect",
                "noreply_wait", "server", "event_emitter", "run",
            ];
            for cmd in blacklist {
                if cmd == command.permalink {
                    return false;
                }
            }
            true
        })
        // Rename special keywords
        .map(|mut command| {
            let keywords = vec!["mod", "match", "do"];
            for cmd in keywords {
                if cmd == command.permalink {
                    command.method = Some(format!("{}_", command.permalink));
                }
            }
            command
        })
        // Rename commands
        .map(|mut command| {
            let names = vec![("to_json_string", "to_json")];
            for (old, new) in names {
                if old == command.permalink {
                    command.method = Some(new.into());
                }
            }
            command
        })
        // Commands with different types
        .map(|mut command| {
            let types = vec![("js", "javascript"), ("do", "funcall")];
            for (cmd, typ) in types {
                if cmd == command.permalink {
                    command.typ = Some(typ.into());
                }
            }
            command
        })
        // Commands in different sections
        .map(|mut command| {
            let menu = vec![("changes", "manipulating-tables")];
            for (cmd, section) in menu {
                if cmd == command.permalink {
                    command.section = section.to_owned();
                }
            }
            command
        })
        .collect();

        Config {
            docs_dir: PathBuf::from(&docs_dir),
            cmds_src: PathBuf::from(&cmds_src),
            version: version,
            menu: commands,
        }
    }
}

fn build_menu(menu_file: &str) -> Vec<Menu> {
    let file = File::open(&menu_file).unwrap();
    let buf = BufReader::new(file);
    yaml::from_reader(buf).unwrap()
}
