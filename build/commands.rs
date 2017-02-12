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

            mod args;
            #[cfg(feature = "with_io")]
            mod io;
            #[cfg(feature = "with_io")]
            pub use self::io::*;

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

        let no_args = format!("{}()", self.menu.name);
        self.tokens = if docs.contains(&no_args) {
            format!(r#"
                pub fn {0}(&self) -> Client {{
                    cmd("{0}")
                }}
            "#, name)
        } else {
            format!(r#"
                pub fn {0}<T: ToArg>(&self, args: T) -> Client {{
                    cmd_with_args("{0}", args)
                }}
            "#, name)
        };
    }
}
