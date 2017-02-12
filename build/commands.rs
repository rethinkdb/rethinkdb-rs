use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use config;
use quote::{Tokens, ToTokens};
use syn::Ident;

#[derive(Debug, Clone)]
pub struct Commands {
    header: Tokens,
    commands: Tokens,
}

#[derive(Debug, Clone)]
pub struct Command {
    menu: config::Command,
    tokens: Tokens,
    pub src: PathBuf,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            header: Self::header(),
            commands: Tokens::new(),
        }
    }

    fn header() -> Tokens {
        let mut header = Tokens::new();

        let token = quote! {
            // AUTO GENERATED
            // Edit in `build/commands.rs` instead

            mod args;
            #[cfg(feature = "with_io")]
            mod io;
            #[cfg(feature = "with_io")]
            pub use self::io::*;

            use Client;
            use ql2::proto::Term;
            use protobuf::repeated::RepeatedField;
            use ql2::proto::Term_TermType;
        };
        token.to_tokens(&mut header);

        header
    }

    pub fn add_command(&mut self, cmd: &Command) {
        cmd.tokens.to_tokens(&mut self.commands);
    }

    pub fn generate<P: AsRef<Path>>(&self, path: P) {
        let header = &self.header;
        let commands = &self.commands;

        let src = quote! {
            #header

            impl Client {
                #commands
            }
        };

        let commands = format!("{}", src);
        let mut file = File::create(path).unwrap();
        file.write_all(commands.as_bytes()).unwrap();
        file.sync_all().unwrap();
    }
}

impl Command {
    pub fn new(dir: &str, menu: config::Command) -> Command {
        let path = format!("{}/{}.md", dir, menu.permalink);
        let src = PathBuf::from(&path);

        let mut cmd = Command {
            menu: menu,
            tokens: Tokens::new(),
            src: src,
        };

        cmd.build();
        cmd
    }

    fn build(&mut self) {
        let name_str = if let Some(ref name) = self.menu.method {
            name
        } else {
            &self.menu.permalink
        };
        
        let name = Ident::new(name_str.as_str());

        let tokens = quote! {
            pub fn #name(&self) -> Client {
                unimplemented!();
            }
        };

        tokens.to_tokens(&mut self.tokens);
    }
}
