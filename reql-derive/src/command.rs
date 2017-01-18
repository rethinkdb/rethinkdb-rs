use quote::{Tokens, ToTokens};
use syn::{Ident, Attribute, MacroInput, VariantData};
use syn::Body::{Struct, Enum};
use case::CaseExt;

pub struct Command {
    ast: MacroInput,
}

impl Command {
    pub fn new(ast: MacroInput) -> Command {
        Command {
            ast: ast,
        }
    }

    pub fn derive(&self) -> Tokens {
        let title = self.title();
        let typ = self.typ();
        let docs = self.docs();
        let sig = self.sig();
        let body = self.body();

        quote! {
            #title
            pub trait #typ {
                #docs
                fn #sig;
            }

            impl #typ for ::Command {
                fn #sig {
                    #body
                }
            }
        }
    }

    fn command(&self) -> &Attribute {
        let mut commands = self.ast.attrs.iter().filter(|attr| {
                if let MetaItem::List(ref attr, _) = attr.value {
                    attr == "command"
                } else {
                    false
                }
            });
        let command = match commands.next() {
            Some(ref command) => command,
            None => panic!("command not defined"),
        };
        if commands.next().is_some() {
            panic!("more than 1 command attributes found");
        }
        let command = match command.value {
            MetaItem::List(_, ref items) => {
                if items.len() != 1 {
                    panic!("A command attribute must only carry one command.");
                }
                let command = items.next().as_ref().unwrap();
                match command {
                    MetaItem(ref command) => command,
                    Literal(_) => panic!("A command attribute should not define commands as literals"),
                }
            }
            _ => panic!("A command attribute must be defined as a list of items. That is `#[command(..)]`."),
        };
        command
    }

    fn name(&self) -> &Ident {
        match self.command() {
            MetaItem::Word(ref name) => name,
            MetaItem::List(ref name, _) => name,
            NameValue(_) => panic!("A command must not be a name value pair"),
        }
    }

    fn value(&self) -> Option<&NestedMetaItem> {
        match self.command() {
            MetaItem::Word(_) => None,
            MetaItem::List(_, ref value) => Some(value),
            NameValue(_) => panic!("A command must not be a name value pair"),
        }
    }

    fn args(&self) -> Vec<(Ident, Ident)> {
        let mut args = Vec::new();
        if let Some(MetaItem::List(ref name, ref value)) = self.value() {
            if name == "args" {
                match *value {
                    NestedMetaItem::MetaItem(ref item) => {
                        if let MetaItem::NameValue(ref typ, ref name) = item {
                            match *name {
                                Lit::Str(ref name, _) => {
                                    args.push((typ.clone(), Ident::new(name)));
                                }
                                _ => panic!("An arg name must be defined as a string"),
                            };
                        } else {
                            panic!("Args must be key value pairs"),
                        }
                    },
                    Literal(_) => panic!("Args must not be literals"),
                }
            }
        }
        args
    }

    fn title(&self) -> Tokens {
        quote!()
    }

    fn typ(&self) -> Tokens {
        let name = self.name().to_string().to_camel();
        let typ = Ident::new(name);
        quote!(#typ)
    }

    fn docs(&self) -> Tokens {
        let mut docs = Tokens::new();
        for attr in self.ast.attrs.iter() {
            if attr.is_sugared_doc {
                attr.to_tokens(&mut docs);
            }
        }
        docs
    }

    fn sig(&self) -> Tokens {
        let func = self.name().clone();

        let mut generics = Tokens::new();
        let mut args = Tokens::new();
        let mut _where = Tokens::new();

        for (typ, arg) in self.args {
        }

        quote! {
            fn #func<L, R>(self, #left_arg: L, #right_arg: R) -> ::Command where L: ::IntoArg, R: ::IntoArg
        }
    }

    fn body(&self) -> Tokens {
        let cmd_type = Ident::new(self.name().to_string().to_snake().to_uppercase());
        let mut args = Tokens::new();
        for (_, arg) in self.args() {
            let token = quote! {
                for arg in #arg.into_arg() {
                    term.mut_args().push(arg);
                }
            };
            token.to_tokens(&mut args);
        }

        quote! {
            let mut term = ::ql2::proto::Term::new();
            term.set_field_type(::ql2::proto::Term_TermType::#cmd_type);
            if let Some(cmd) = self.term {
                let args = ::protobuf::repeated::RepeatedField::from_vec(vec![cmd]);
                term.set_args(args);
            }
            #args
            ::Command {
                term: Some(term),
                idx: self.idx + 1,
            }
        }
    }
}

pub fn expand(ast: &MacroInput) -> Tokens {
    let info = match ast.body {
        Struct(ref data) => {
            if let &VariantData::Unit = data {
                Info {
                    arg_name: None,
                    min_max_arg: false,
                    left_arg: None,
                    right_arg: None,
                }
            }
            else { panic!("only unit structs and enums are supported"); }
        }
        Enum(ref vars) => {
            let mut arg_name = None;
            let mut min_max_arg = false;
            let mut left_arg = None;
            let mut right_arg = None;

            for var in vars.iter() {
                let label = var.ident.to_string();
                match label.as_str() {
                    s if s.starts_with("Argname") => {
                        let name = s.trim_left_matches("Argname").to_snake();
                        if name.is_empty() {
                            panic!("`Argname` must be followed by the name of the argument eg. ArgnameDbName");
                        }
                        let name = Ident::new(name);
                        arg_name = Some(name);
                    }
                    s if s.starts_with("Leftarg") => {
                        let name = s.trim_left_matches("Leftarg").to_snake();
                        if name.is_empty() {
                            panic!("`Leftarg` must be followed by the name of the argument eg. LeftargDbName");
                        }
                        let name = Ident::new(name);
                        left_arg = Some(name);
                    }
                    s if s.starts_with("Rightarg") => {
                        let name = s.trim_left_matches("Rightarg").to_snake();
                        if name.is_empty() {
                            panic!("`Rightarg` must be followed by the name of the argument eg. RightargDbName");
                        }
                        let name = Ident::new(name);
                        right_arg = Some(name);
                    }
                    "MinMaxArg" => {
                        min_max_arg = true;
                    }
                    _ => {
                        let msg = format!("Enum {} has label {} which is unknown", ast.ident, label);
                        panic!(msg);
                    }
                }
            }

            Info {
                arg_name: arg_name,
                min_max_arg: min_max_arg,
                left_arg: left_arg,
                right_arg: right_arg,
            }
        }
    };

    // Extract name of command
    let label = ast.ident.to_string();
    let name = label.trim_left_matches("_");

    // Create the identifiers
    let typ = Ident::new(name);
    let func = Ident::new(name.to_snake());
    let cmd_type = Ident::new(name.to_snake().to_uppercase());

    let mut tokens = Tokens::new();

    // Declare the trait
    let token = quote! { pub trait #typ };
    token.to_tokens(&mut tokens);
    tokens.append("{");

    // Add documentation, if any
    for attr in ast.attrs.iter() {
        if attr.is_sugared_doc {
            attr.to_tokens(&mut tokens);
        }
    }

    // Finish defining the trait
    let sig = match info.arg_name {
        Some(ref name) => {
            let name = name.clone();
            quote! { fn #func<T>(self, #name: T) -> ::Command where T: ::IntoArg }
        }
        None => {
            if info.min_max_arg {
                quote! { fn #func<T>(self, min: T, max: T) -> ::Command where T: ::IntoArg }
            } else {
                if let Some(ref name) = info.left_arg {
                    let left_arg = name.clone();
                    let right_arg = info.right_arg.clone()
                        .expect(&format!("{}: Rightarg can't be empty when Leftarg isn't", ast.ident));
                    quote! { fn #func<L, R>(self, #left_arg: L, #right_arg: R) -> ::Command where L: ::IntoArg, R: ::IntoArg }
                } else {
                    quote! { fn #func (self) -> ::Command }
                }
            }
        }
    };

    sig.clone().to_tokens(&mut tokens);
    tokens.append(";");
    tokens.append("}");

    // Implement it
    let token = quote! { impl #typ for ::Command };
    token.to_tokens(&mut tokens);
    tokens.append("{");

    sig.to_tokens(&mut tokens);
    tokens.append("{");

    let token = quote! {
        let mut term = ::ql2::proto::Term::new();
        term.set_field_type(::ql2::proto::Term_TermType::#cmd_type);
        if let Some(cmd) = self.term {
            let args = ::protobuf::repeated::RepeatedField::from_vec(vec![cmd]);
            term.set_args(args);
        }
    };
    token.to_tokens(&mut tokens);

    if let Some(ref name) = info.arg_name {
        let name = name.clone();
        let token = quote! {
            for arg in #name.into_arg() {
                term.mut_args().push(arg);
            }
        };
        token.to_tokens(&mut tokens);
    }

    if let Some(ref name) = info.left_arg {
        let left_arg = name.clone();
        let right_arg = info.right_arg.unwrap();
        let token = quote! {
            for arg in #left_arg.into_arg() {
                term.mut_args().push(arg);
            }
            for arg in #right_arg.into_arg() {
                term.mut_args().push(arg);
            }
        };
        token.to_tokens(&mut tokens);
    }

    if info.min_max_arg {
        let token = quote! {
            for arg in min.into_arg() {
                term.mut_args().push(arg);
            }
            for arg in max.into_arg() {
                term.mut_args().push(arg);
            }
        };
        token.to_tokens(&mut tokens);
    }

    let token = quote! {
        ::Command {
            term: Some(term),
            idx: self.idx + 1,
        }
    };
    token.to_tokens(&mut tokens);

    tokens.append("}");
    tokens.append("}");

    tokens
}
