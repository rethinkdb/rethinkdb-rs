use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use config;
use unindent::unindent;

#[derive(Debug, Clone)]
pub struct Commands {
    header: String,
    commands: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Command {
    menu: config::Command,
    tokens: String,
    pub src: PathBuf,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            header: Self::header(),
            commands: Vec::new(),
        }
    }

    fn header() -> String {
        let cmd = Self::cmd();
        let cmd_with_args = Self::cmd_with_args();

        format!(r#"
            // AUTO GENERATED
            // Edit in `build/commands.rs` instead

            /*
            mod args;
            #[cfg(feature = "with_io")]
            mod io;
            #[cfg(feature = "with_io")]
            pub use self::io::*;
            */

            use {{Client, ToArg}};
            use ql2::proto::Term;
            use protobuf::repeated::RepeatedField;
            use ql2::proto::Term_TermType;

            {}
            
            {}
        "#, cmd, cmd_with_args)
    }

    pub fn add_command(&mut self, cmd: &Command) {
        self.commands.push(cmd.tokens.to_owned());
    }

    pub fn generate<P: AsRef<Path>>(&self, path: P) {
        let header = &self.header;
        let commands: String = self.commands.join("\n");

        let src = format!(r#"
            {}

            impl Client {{
                {}
            }}
        "#, header, commands);

        let mut file = File::create(path).unwrap();
        file.write_all(unindent(&src).as_bytes()).unwrap();
        file.sync_all().unwrap();
    }

    fn cmd() -> String {
        format!(r#"
            fn cmd(name: &str) -> Client {{
                unimplemented!();
            }}
        "#)
    }

    fn cmd_with_args() -> String {
        format!(r#"
            fn cmd_with_args<T: ToArg>(name: &str, args: T) -> Client {{
                unimplemented!();
            }}
        "#)
    }
}

impl Command {
    pub fn new(dir: &str, menu: config::Command) -> Command {
        let path = format!("{}/{}.md", dir, menu.permalink);
        let src = PathBuf::from(&path);

        let mut cmd = Command {
            menu: menu,
            tokens: String::new(),
            src: src,
        };

        cmd.build();
        cmd
    }

    fn build(&mut self) {
        let name = if let Some(ref name) = self.menu.method {
            name
        } else {
            &self.menu.permalink
        };
        
        let mut docs = String::new();
        let mut file = File::open(&self.src).unwrap();
        if file.read_to_string(&mut docs).unwrap() == 0 {
            panic!(format!("command file is empty: {:?}", self));
        }

        let (no_args, docs) = self.gen_docs(docs);
        self.tokens = if no_args {
            format!(r#"
                {1}
                pub fn {0}(&self) -> Client {{
                    cmd("{0}")
                }}
            "#, name, docs)
        } else {
            format!(r#"
                {1}
                pub fn {0}<T: ToArg>(&self, args: T) -> Client {{
                    cmd_with_args("{0}", args)
                }}
            "#, name, docs)
        };
    }

    fn gen_docs(&self, mut docs: String) -> (bool, String) {
        let mut no_args = false;
        let cmd = format!("{}()", self.menu.name);

        let mut doc_block = false;
        let mut parse = false;
        let mut img = String::new();
        let mut title = String::new();
        let mut next_line = String::new();

        docs = docs.lines()
            // If the command is documented with no args
            // we won't give it args
            .map(|line| {
                if line.contains(&cmd) {
                    no_args = true;
                }
                line
            })
            // We will only consider docs after the description
            .filter(|line| {
                if !doc_block {
                    if line.starts_with("# Description #") {
                        doc_block = true;
                    }
                    return false;
                }
                if !parse {
                    if line.trim().is_empty() {
                        return false;
                    }
                    if line.starts_with("<img src=") {
                        img = line.replace("/assets/images/", "https://rethinkdb.com/assets/images/");
                        return false;
                    }
                    if let Some(i) = line.find('.') {
                        let (t, n) = line.split_at(i);
                        title = t.to_owned();
                        next_line = n.trim_left_matches('.').trim().to_owned();
                        parse = true;
                        return false;
                    }
                    parse = true;
                }
                true
            })
            .map(|line| {
                // Indent commands so they come out nice
                format!("                /// {}\n", line)
            })
        .collect();

        let docs = format!("/// {}
            ///
            /// {}
            ///
            /// {}
            {}", title, img, next_line, docs);

        (no_args, docs)
    }
}
