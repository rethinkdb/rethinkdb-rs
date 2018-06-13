use config;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/build/parsers.rs"));

#[derive(Debug, Clone)]
pub struct Commands {
    header: String,
    commands: Vec<String>,
    menu: Vec<config::Command>,
}

#[derive(Debug, Clone)]
pub struct Command {
    menu: config::Command,
    tokens: String,
    pub src: PathBuf,
}

impl Commands {
    pub fn new(menu: &[config::Command]) -> Commands {
        Commands {
            header: Self::header(),
            commands: Vec::new(),
            menu: menu.to_owned(),
        }
    }

    fn header() -> String {
        format!(r#"
            // AUTO GENERATED
            // Manual changes made to this file will be overwritten by the build script.
            // Edit `build/commands.rs` instead...
            // @generated

            mod io;
            mod util;
            mod args;

            use Connection;
            use {{Config, Client, IntoArg, Result}};
            use ql2::proto::{{Term, Term_TermType as Type}};
        "#)
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

                /// Create a new ReQL client
                ///
                /// By convention, the variable binding holding a ReQL client is called `r`.
                ///
                /// __Example__: Create your client.
                ///
                /// ```reql
                /// let r = Client::new();
                /// ```

                pub fn new() -> Client {{
                    util::new_client()
                }}

                #[doc(hidden)]
                pub fn set_term(&mut self, term: Result<Term>) {{
                    self.term = term;
                }}

                #[doc(hidden)]
                pub fn term(&self) -> Result<&Term> {{
                    match self.term {{
                        Ok(ref term) => Ok(term),
                        Err(ref error) => Err(error.clone()),
                    }}
                }}

                /// Specify optional arguments to a ReQL command
                ///
                /// Normally, you should use the `args!()` macro to pass arguments to a command that
                /// also takes optional arguments. If the command takes at least one argument, you
                /// don't need to call `with_args`. However, some commands like [delete](struct.Client.html#method.delete)
                /// do not have any required arguments but yet they have optional ones. That's when `with_args` comes in.
                /// The `args` macro is provided by the `reql-macros` crate. NB: That crate
                /// requires the nightly compiler. See its docs for more details.
                ///
                /// __Example__: Delete all documents from the table `comments` without waiting for the operation to be flushed to
                /// disk.
                ///
                /// ```rust,ignore
                /// # #![feature(proc_macro)]
                /// # #![feature(proc_macro_non_items)]
                /// # #![allow(unused_must_use)]
                /// # extern crate reql;
                /// # extern crate reql_macros;
                /// # use reql_macros::args;
                /// # fn main() {{
                /// # use reql::Client;
                /// # let r = Client::new();
                /// r.table("comments").delete().with_args(args!({{durability: "soft"}}));
                /// # }}
                /// ```

                pub fn with_args<T: IntoArg>(&self, args: T) -> Client {{
                    util::with_args(self, args)
                }}

                {}
            }}
        "#, header, commands);

        let mut file = File::create(path).unwrap();
        file.write_all(src.as_bytes()).unwrap();
        file.sync_all().unwrap();
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

        let cmd = if let Some(ref cmd) = self.menu.typ {
            cmd
        } else {
            &self.menu.permalink
        };

        let typ = format!("Type::{}", cmd.to_uppercase());

        let mut docs = String::new();
        let mut file = File::open(&self.src).unwrap();
        if file.read_to_string(&mut docs).unwrap() == 0 {
            panic!(format!("command file is empty: {:?}", self));
        }

        let (no_args, docs) = self.gen_docs(docs);
        self.tokens = if name == "connect" {
            format!(r#"
                {}
                pub fn connect<'a>(&self, cfg: Config<'a>) -> Result<Connection> {{
                    io::connect(self, cfg)
                }}
            "#,
                    docs)
        } else if name == "expr" {
            format!(r#"
                {}
                pub fn expr<T: IntoArg>(&self, args: T) -> Client {{
                    util::make_cmd(self, "expr", None, Some(args))
                }}
            "#,
                    docs)
        } else if no_args {
            format!(r#"
                {docs}
                pub fn {name}(&self) -> Client {{
                    util::make_cmd::<Client>(self, "{name}", Some({typ}), None)
                }}
            "#,
                    docs = docs,
                    name = name,
                    typ = typ)
        } else {
            format!(r#"
                {docs}
                pub fn {name}<T: IntoArg>(&self, args: T) -> Client {{
                    util::make_cmd(self, "{name}", Some({typ}), Some(args))
                }}
            "#,
                    docs = docs,
                    name = name,
                    typ = typ)
        };
    }

    fn gen_docs(&self, mut docs: String) -> (bool, String) {
        let mut no_args = false;
        let cmd = format!("{}()", self.menu.name);

        let mut doc_block = false;
        let mut parse = false;
        let mut doc_str = String::new();
        let mut img = String::new();
        // The sentence following the title
        let mut next_line = String::new();

        docs = docs.lines()
            // If the command is documented with no args
            // we won't give it args
            .map(|line| {
                if line.contains(&cmd) {
                    no_args = true;
                }
                self.fixup(line)
            })
            .filter(|line| {
                // If we haven't started parsing yet, ignore empty lines
                // and grab any image we find
                if !parse {
                    if line.trim().is_empty() {
                        return false;
                    }
                    if line.starts_with("<img src=") {
                        img = line.to_owned();
                        return false;
                    }
                }
                // We will only consider docs after the description
                if !doc_block {
                    if line.starts_with("# Description #") {
                        doc_block = true;
                    }
                    return false;
                }
                // Grab the title and start parsing
                if !parse {
                    if let Some(i) = line.find('.') {
                        let (t, n) = line.split_at(i);
                        doc_str.push_str(&format!("/// {}\n", t));
                        next_line = n.trim_left_matches('.').trim().to_owned();
                        parse = true;
                        return false;
                    } else {
                        doc_str.push_str(&format!("/// {}\n", line));
                        return false;
                    }
                }
                true
            })
            .map(|line| {
                // Indent commands so they come out nice
                format!("/// {}\n", line)
            })
        .collect();

        if !img.is_empty() {
            doc_str.push_str(&format!("///\n/// {}\n", img));
        }

        if !next_line.is_empty() {
            doc_str.push_str(&format!("///\n/// {}\n", next_line));
        }

        if !docs.is_empty() {
            doc_str.push_str("///\n");
            doc_str.push_str(&docs);
        }

        (no_args, doc_str)
    }

    fn fixup(&self, commands: &str) -> String {
        commands.lines()
            .map(|line| {
                line.replace("/assets/images/docs/", "https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/")
            })
            .collect()
    }
}
