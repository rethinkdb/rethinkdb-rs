use quote::{Tokens, ToTokens};
use syn::{Ident, MacroInput, VariantData};
use syn::Body::{Struct, Enum};
use case::CaseExt;

struct Info {
    arg_name: Option<Ident>,
    min_max_arg: bool,
    left_arg: Option<Ident>,
    right_arg: Option<Ident>,
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
            use ::IntoArg;
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
            use ::IntoArg;
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
