use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

pub(super) fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut methods = TokenStream::new();

    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                for field in fields.named {
                    let name = field.ident;
                    let mut generics = TokenStream::new();
                    let mut where_clause = TokenStream::new();
                    let mut value = quote!(#name);
                    let mut param = param(&field.ty);

                    if param.is_cow || param.is_db {
                        generics = quote!(<T>);
                        where_clause = quote!(where T: crate::cmd::StaticString);
                        param.ty = quote!(T);
                        value = quote!(#name.static_string());
                    }

                    if param.is_db {
                        value = quote!(Db(#value))
                    }

                    if param.is_option {
                        value = quote!(Some(#value))
                    }

                    let ty = param.ty;

                    methods.extend(quote! {
                        pub fn #name #generics(mut self, #name: #ty) -> Self #where_clause {
                            self.#name = #value;
                            self
                        }
                    });
                }
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }

    let name = input.ident;
    let generics = input.generics;

    let options = quote! {
        impl #generics #name #generics {
            pub fn new() -> Self {
                Default::default()
            }

            #methods
        }
    };

    options.into()
}

struct Param {
    ty: TokenStream,
    is_option: bool,
    is_cow: bool,
    is_db: bool,
}

fn param(typ: &Type) -> Param {
    if let Type::Path(typ) = typ {
        if let Some(typ) = typ.path.segments.first() {
            let mut param = Param {
                ty: typ.to_token_stream(),
                is_option: typ.ident == "Option",
                is_cow: typ.ident == "Cow",
                is_db: false,
            };
            if !param.is_option {
                return param;
            } else if let PathArguments::AngleBracketed(path) = &typ.arguments {
                if let Some(typ) = path.args.first() {
                    param.ty = typ.to_token_stream();
                    if let GenericArgument::Type(Type::Path(typ)) = typ {
                        if let Some(typ) = typ.path.segments.first() {
                            param.is_cow = typ.ident == "Cow";
                            param.is_db = typ.ident == "Db";
                        }
                    }
                    return param;
                }
            }
        }
    }
    panic!("{:#?}", typ);
}
