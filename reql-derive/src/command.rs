use std::collections::HashMap;

use quote::{Tokens, ToTokens};
use syn::{Ident, Lit, MetaItem, NestedMetaItem, MacroInput};
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
        let typ = self.typ();
        let docs = self.docs();
        let sig = self.sig();
        let body = self.body();
        let note = self.note();
        let related = self.related();

        quote! {
            #docs
            #note
            #related
            pub trait #typ {
                fn #sig;
            }

            impl #typ for ::Command {
                fn #sig {
                    #body
                }
            }
        }
    }

    fn note(&self) -> Tokens {
        if self.docs().to_string().contains("Example") {
            return Tokens::new();
        }

        let msg = format!(r" **Note:** This command is not yet fully documented. For more information and examples, please see [the equivalent command](https://rethinkdb.com/api/java/{}/) in the official Java driver documentation.", self.name());

        quote!{
            #[doc=r""]
            #[doc=#msg]
        }
    }

    fn related(&self) -> Tokens {
        let mut related = Tokens::new();
        for item in self.value() {
            if let NestedMetaItem::MetaItem(MetaItem::List(ref name, ref value)) = *item {
                if name == "related" {
                    related.append_all(&[quote!{
                        ///
                        /// # Related commands
                        ///
                    }]);
                    for item in value {
                        match *item {
                            NestedMetaItem::MetaItem(ref item) => {
                                if let MetaItem::Word(ref cmd) = *item {
                                    let item = format!(r" * [{}](trait.{}.html)", cmd, cmd.as_ref().to_camel());
                                    related.append_all(&[quote!(#[doc=#item])]);
                                } else {
                                    panic!("Related commands must be identifiers");
                                }
                            },
                            NestedMetaItem::Literal(_) => { panic!("Related commands must not be literals"); },
                        }
                    }
                }
            }
        }
        related
    }

    fn command(&self) -> &MetaItem {
        let mut commands = self.ast.attrs.iter().filter(|attr| {
                if let MetaItem::List(ref attr, _) = attr.value {
                    attr == "command"
                } else {
                    false
                }
            })
        .peekable();
        let command = commands.peek().expect("command attribute not defined");
        let mut meta_items = Vec::new();
        match command.value {
            MetaItem::List(_, ref items) => {
                if items.len() != 1 {
                    panic!("A command attribute must only carry one command.");
                }
                for item in items {
                    match *item {
                        NestedMetaItem::MetaItem(ref command) => { meta_items.push(command); },
                        NestedMetaItem::Literal(_) => { panic!("A command attribute should not define commands as literals"); },
                    }
                }
            }
            _ => { panic!("A command attribute must be defined as a list of items. That is `#[command(..)]`."); },
        };
        meta_items.iter().next().unwrap()
    }

    fn name(&self) -> &Ident {
        match *self.command() {
            MetaItem::Word(ref name) => name,
            MetaItem::List(ref name, _) => name,
            MetaItem::NameValue(..) => { panic!("A command must not be a name value pair"); },
        }
    }

    fn value(&self) -> Vec<&NestedMetaItem> {
        let mut value = Vec::new();
        match *self.command() {
            MetaItem::Word(_) => { },
            MetaItem::List(_, ref items) => {
                for item in items {
                    value.push(item);
                }
            }
            MetaItem::NameValue(..) => { panic!("A command must not be a name value pair"); },
        }
        value
    }

    fn args(&self) -> Vec<(Ident, Ident)> {
        let mut args = Vec::new();
        for item in self.value() {
            if let NestedMetaItem::MetaItem(MetaItem::List(ref name, ref value)) = *item {
                if name == "args" {
                    for item in value {
                        match *item {
                            NestedMetaItem::MetaItem(ref item) => {
                                if let MetaItem::NameValue(ref name, ref typ) = *item {
                                    match *typ {
                                        Lit::Str(ref typ, _) => {
                                            args.push((name.clone(), Ident::new(typ.as_str())));
                                        }
                                        _ => { panic!("An arg name must be defined as a string"); },
                                    };
                                } else {
                                    panic!("Args must be key value pairs");
                                }
                            },
                            NestedMetaItem::Literal(_) => { panic!("Args must not be literals"); },
                        }
                    }
                }
            }
        }
        args
    }

    fn typ(&self) -> Tokens {
        let name = self.name().to_string().to_camel();
        let typ = Ident::new(name);
        quote!(#typ)
    }

    fn docs(&self) -> Tokens {
        let mut docs = Tokens::new();
        for attr in self.ast.attrs.iter() {
            if let MetaItem::NameValue(ref name, ref value) = attr.value {
                if name == "doc" {
                    if let Lit::Str(ref doc_str, _) = *value {
                        if doc_str.contains("```reql") {
                            let macro_use = if doc_str.contains("```reql,macros") {
                                quote! {
                                    /// # #[macro_use] extern crate reql;
                                }
                            } else {
                                Tokens::new()
                            };
                            let token = quote! {
                                /// ```
                                #macro_use
                                /// # use reql::commands::*;
                                /// # use reql::commands::run::Dummy;
                                /// # use reql::r;
                            };
                            token.to_tokens(&mut docs);
                        } else {
                            attr.to_tokens(&mut docs);
                        }
                    }
                }
            }
        }
        docs
    }

    fn sig(&self) -> Tokens {
        let mut types = HashMap::new();

        let func = self.name().clone();
        let mut generics = Tokens::new();
        let mut func_args = Tokens::new();
        let mut _where = Tokens::new();

        let args = self.args();
        if !args.is_empty() {
            let mut gen = Vec::new();
            let mut whe = Vec::new();

            gen.push(quote!(<));
            whe.push(quote!(where));

            for (arg, typ) in args {
                let typ_str = typ.to_string();
                if let None = types.get(&typ_str) {
                    gen.push(quote!(#typ));
                    gen.push(quote!(,));
                    whe.push(quote!(#typ: ::ToArg));
                    whe.push(quote!(,));
                    types.insert(typ_str, ());
                }
                func_args.append_all(&[ quote!(, #arg: #typ) ]);
            }

            gen.pop();
            whe.pop();

            gen.push(quote!(>));

            generics.append_all(gen);
            _where.append_all(whe);
        }

        quote! {
            #func #generics (&self #func_args) -> ::Command #_where
        }
    }

    fn body(&self) -> Tokens {
        // Rename commands to term types
        let cmd = match self.name() {
            name if name == "match_" => "match".to_string(),
            name if name == "mod_" => "mod".to_string(),
            name if name == "do_" => "funcall".to_string(),
            name if name == "js" => "javascript".to_string(),
            name if name == "to_json" => "to_json_string".to_string(),
            name => name.to_string(),
        };
        // Prepare args
        let mut args = Tokens::new();
        for (arg, _) in self.args() {
            let token = quote! {
                term.mut_args().push(#arg.to_arg());
            };
            token.to_tokens(&mut args);
        }
        let cmd_type = Ident::new(cmd.to_snake().to_uppercase());
        // Build the body
        quote! {
            use ::ql2::proto::Term;
            use ::protobuf::repeated::RepeatedField;
            use ::ql2::proto::Term_TermType;

            let mut term = Term::new();
            term.set_field_type(Term_TermType::#cmd_type);
            if self.term != Term::new() {
                let prev_cmd = RepeatedField::from_vec(vec![self.term.clone()]);
                term.set_args(prev_cmd);
            }
            #args
            let mut cmd = ::Command::new();
            cmd.set_term(term);
            cmd
        }
    }
}
