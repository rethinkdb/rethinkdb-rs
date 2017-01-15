use quote::{Tokens, ToTokens};
use syn::{Ident, MacroInput, VariantData};
use syn::Body::{Struct, Enum};

struct Info {
    arg_name: Option<Ident>,
    min_max_arg: bool,
}

pub fn expand(ast: &MacroInput) -> Tokens {
    let info = match ast.body {
        Struct(ref data) => {
            if let &VariantData::Unit = data {
                Info {
                    arg_name: None,
                    min_max_arg: false,
                }
            }
            else { panic!("only unit structs and enums are supported"); }
        }
        Enum(ref vars) => {
            let mut arg_name = None;
            let mut min_max_arg = false;

            for var in vars.iter() {
                let label = var.ident.to_string();
                match label.as_str() {
                    s if s.starts_with("Argname") => {
                        let name = s.trim_left_matches("Argname").to_lowercase();
                        if name.is_empty() {
                            panic!("`Argname` must be followed by the name of the argument eg. ArgnameDbName");
                        }
                        let name = Ident::new(name);
                        arg_name = Some(name);
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
            }
        }
    };

    // Extract name of command
    let label = ast.ident.to_string();
    let name = label.trim_left_matches("_");

    // Create the identifiers
    let typ = Ident::new(name);
    let func = Ident::new(name.to_lowercase());
    let cmd_type = Ident::new(name.to_uppercase());

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
            quote! { fn #func<T: ::IntoArg>(self, #name: T) -> ::Command }
        }
        None => {
            if info.min_max_arg {
                quote! { fn #func<T: ::IntoArg>(self, min: T, max: T) -> ::Command }
            } else {
                quote! { fn #func (self) -> ::Command }
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
            use ::IntoArg;
            for arg in #name.into_arg() {
                term.mut_args().push(arg);
            }
        };
        token.to_tokens(&mut tokens);
    }

    if info.min_max_arg {
        let token = quote! {
            use ::IntoArg;
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
