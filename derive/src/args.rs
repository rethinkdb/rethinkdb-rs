use syn::{self, Token, TokenTree, Ident, BinOpToken, DelimToken};
use quote::{Tokens, ToTokens};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Args<'a> {
    input: &'a str,
    pub tokens: Tokens,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Expr,
    Closure,
    List,
    Object,
}

#[derive(Debug, Clone)]
enum Group {
    Expr(Vec<TokenTree>),
    Closure(Vec<TokenTree>),
    List(Vec<Group>),
    Object(HashMap<String, Group>),
}

impl<'a> Args<'a> {
    pub fn new(input: &str) -> Args {
        Args {
            input: input.trim(),
            tokens: Tokens::new(),
        }
    }

    pub fn process(&mut self) -> &mut Self {
        if self.input.is_empty() {
            self.tokens = quote!(Term::new());
            return self;
        }

        let args = format!("args!({})", self.input);
        let body = self.body();

        self.tokens = quote!({
            let mut args = Arg::new();
            args.set_string(#args);
            #body
            args
        });

        self
    }

    fn body(&mut self) -> Tokens {
        let tt = syn::parse_token_trees(self.input).expect("failed to parse token tree");
        let mut tokens = Tokens::new();
        let args = self.group_by_comma(tt);
        let last = args.len()-1;
        for (i, arg) in args.into_iter().enumerate() {
            arg.tokenise("args", i == last).to_tokens(&mut tokens);
        }
        tokens
    }

    fn finalise_arg(&self, args: &mut Vec<Group>, tokens: &mut Vec<TokenTree>, typ: &mut Type, found_lbar: &mut bool, found_rbar: &mut bool, found_body: &mut bool) {
        let group = match *typ {
            Type::Expr => Group::Expr(tokens.clone()),
            Type::Closure => Group::Closure(tokens.clone()),
            Type::List => Group::List(self.group_by_comma(tokens.clone())),
            Type::Object => Group::Object(self.hash_from_tokens(tokens.clone())),
        };
        args.push(group);
        *tokens = Vec::new();
        *typ = Type::Expr;
        *found_lbar = false;
        *found_rbar = false;
        *found_body = false;
    }

    fn hash_from_tokens(&self, tt: Vec<TokenTree>) -> HashMap<String, Group> {
        let mut hash = HashMap::new();

        let mut is_key = true;
        let mut key = String::new();
        let mut val = Vec::new();
        let last = tt.len()-1;

        for (i, tree) in tt.into_iter().enumerate() {
            if let TokenTree::Token(Token::Colon) = tree {
                if key.is_empty() {
                    panic!("A key of an object cannot be empty.");
                }
                is_key = false;
                continue;
            }
            if is_key {
                key = match tree {
                    TokenTree::Token(Token::Literal(syn::Lit::Str(key, _))) => key,
                    tree => {
                        let mut tokens = Tokens::new();
                        tree.to_tokens(&mut tokens);
                        tokens.to_string()
                    }
                };
                continue;
            }
            let token_is_comma = if let TokenTree::Token(Token::Comma) = tree { true } else { false };
            let on_last_token = if i == last { true } else { false };
            let val_end = token_is_comma || on_last_token;
            if !token_is_comma {
                val.push(tree);
            }
            if val_end {
                let arg = self.group_by_comma(val.clone());
                let len = arg.len();
                if len == 0 {
                    panic!("An object key cannot have no value.");
                } else if len == 1 {
                    hash.insert(key.clone(), arg.into_iter().next().unwrap());
                } else {
                    panic!("An object key cannot have more than 1 values.");
                }
                is_key = true;
                val = Vec::new();
            }
        }

        hash
    }

    fn group_by_comma(&self, tt: Vec<TokenTree>) -> Vec<Group> {
        let mut args = Vec::new();
        let mut tokens = Vec::new();
        let mut typ = Type::Expr;
        let last = tt.len()-1;
        let mut found_lbar = false;
        let mut found_rbar = false;
        let mut found_body = false;

        for (i, tree) in tt.into_iter().enumerate() {
            let token_is_comma = if let TokenTree::Token(Token::Comma) = tree { true } else { false };
            let on_last_token = if i == last { true } else { false };
            let arg_end = token_is_comma || on_last_token;

            match Type::new(&tokens, &tree, &mut typ, &mut found_lbar, &mut found_rbar, &mut found_body) {
                Type::Expr => {
                    if !token_is_comma {
                        tokens.push(tree);
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ, &mut found_lbar, &mut found_rbar, &mut found_body);
                    }
                }
                Type::Closure => {
                    if !token_is_comma {
                        tokens.push(tree);
                    }
                    if on_last_token || (found_body && token_is_comma) {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ, &mut found_lbar, &mut found_rbar, &mut found_body);
                    }
                }
                Type::List => {
                    if !token_is_comma {
                        if let TokenTree::Delimited(d) = tree {
                            tokens = d.tts;
                        }
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ, &mut found_lbar, &mut found_rbar, &mut found_body);
                    }
                }
                Type::Object => {
                    if !token_is_comma {
                        if let TokenTree::Delimited(d) = tree {
                            tokens = d.tts;
                        }
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ, &mut found_lbar, &mut found_rbar, &mut found_body);
                    }
                }
            }
        }
        args
    }
}

impl Group {
    fn tokenise(self, var: &str, last: bool) -> Tokens {
        let mut tokens = Tokens::new();
        let var = Ident::from(var);
        match self {
            Group::Expr(tt) => {
                let mut expr = Tokens::new();
                for token in tt {
                    token.to_tokens(&mut expr);
                }
                quote!(#var.add_arg(#expr.into_arg());)
                    .to_tokens(&mut tokens);
            }
            Group::Closure(tt) => {
                let mut func = Tokens::new();
                let mut args = Tokens::new();
                let mut found_lbar = false;
                let mut found_rbar = false;
                for (i, token) in tt.into_iter().enumerate() {
                    token.to_tokens(&mut func);
                    if found_rbar {
                        continue;
                    }
                    if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = token {
                        if found_lbar {
                            found_rbar = true;
                            continue;
                        } else {
                            found_lbar = true;
                            continue;
                        }
                    }
                    else if let TokenTree::Token(Token::OrOr) = token {
                        found_lbar = true;
                        found_rbar = true;
                        continue;
                    }
                    if found_lbar {
                        quote!(: Client,)
                            .to_tokens(&mut func);
                        quote!(var!(#i),)
                            .to_tokens(&mut args);
                    }
                }
                let closure = quote! {
                    let func = func!((#func), #args);
                    #var.add_arg(func.into_arg());
                };
                closure.to_tokens(&mut tokens);
            }
            Group::List(tt) => {
                let mut list = quote!(let mut list_arg = Arg::new(););
                for group in tt {
                    quote!(let mut list_val = Arg::new();)
                        .to_tokens(&mut list);
                    group.tokenise("list_val", false)
                        .to_tokens(&mut list);
                    quote!(list_arg.add_arg(list_val);)
                        .to_tokens(&mut list);
                }
                quote!(#var.add_arg(list_arg);)
                        .to_tokens(&mut list);
                list.to_tokens(&mut tokens);
            }
            Group::Object(tt) => {
                let mut obj = Tokens::new();
                if !last {
                    quote!(let mut obj_arg = Arg::new();)
                        .to_tokens(&mut obj);
                }
                for (key, group) in tt {
                    quote!(let mut obj_val = Arg::new();)
                        .to_tokens(&mut obj);
                    group.tokenise("obj_val", false)
                        .to_tokens(&mut obj);
                    if last {
                        quote!(
                            match Arg::create_term_pair(#key, obj_val) {
                                Ok(temp_pair) => #var.add_opt(temp_pair),
                                Err(error) => #var.set_term(Err(error)),
                            })
                        .to_tokens(&mut obj);
                    } else {
                        quote!(
                            match Arg::create_term_pair(#key, obj_val) {
                                Ok(temp_pair) => obj_arg.add_opt(temp_pair),
                                Err(error) => obj_arg.set_term(Err(error)),
                            })
                        .to_tokens(&mut obj);
                    }
                }
                if !last {
                    quote!(#var.add_arg(obj_arg);)
                        .to_tokens(&mut obj);
                }
                obj.to_tokens(&mut tokens);
            }
        }
        tokens
    }
}

impl Type {
    fn new(tokens: &Vec<TokenTree>, tree: &TokenTree, typ: &mut Type, found_lbar: &mut bool, found_rbar: &mut bool, found_body: &mut bool) -> Type {
        if *typ == Type::Closure {
            if *found_rbar {
                *found_body = true;
            }
            else if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = *tree {
                if *found_lbar {
                    *found_rbar = true;
                } else {
                    *found_lbar = true;
                }
            }
            return *typ;
        }
        if tokens.is_empty() {
            if let TokenTree::Delimited(ref d) = *tree {
                if d.delim == DelimToken::Bracket { *typ = Type::List; }
                else if d.delim == DelimToken::Brace { *typ = Type::Object; }
            }
            else if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = *tree {
                *typ = Type::Closure;
                if *found_lbar {
                    *found_rbar = true;
                } else {
                    *found_lbar = true;
                }
            }
            else if let TokenTree::Token(Token::OrOr) = *tree {
                *typ = Type::Closure;
                *found_lbar = true;
                *found_rbar = true;
            }
        } else if tokens.len() == 1 {
            if let TokenTree::Token(Token::Ident(ref i)) = tokens[0] {
                if i == "move" {
                    if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = *tree {
                        *typ = Type::Closure;
                        if *found_lbar {
                            *found_rbar = true;
                        } else {
                            *found_lbar = true;
                        }
                    }
                    else if let TokenTree::Token(Token::OrOr) = *tree {
                        *typ = Type::Closure;
                        *found_lbar = true;
                        *found_rbar = true;
                    }
                }
            }
        }
        *typ
    }
}
