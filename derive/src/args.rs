use syn::{self, Token, TokenTree, BinOpToken, DelimToken};
use quote::{Tokens, ToTokens};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Args<'a> {
    input: &'a str,
    args: Vec<Group>,
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
            args: Vec::new(),
            tokens: Tokens::new(),
        }
    }

    pub fn process(&mut self) -> &mut Self {
        if self.input.is_empty() {
            self.tokens = quote!(reql::Term::new());
            return self;
        }

        let args = format!("args!({})", self.input);
        let body = self.body();

        self.tokens = quote!({
            let mut args = reql::Args::new();
            args.set_string(#args);
            #body
            args
        });

        self
    }

    fn body(&mut self) -> Tokens {
        let tt = syn::parse_token_trees(self.input).expect("failed to parse token tree");
        self.args = self.group_by_comma(tt);
        panic!(format!("{:?}", self.args));
    }

    fn finalise_arg(&self, args: &mut Vec<Group>, tokens: &mut Vec<TokenTree>, typ: &mut Type) {
        let group = match *typ {
            Type::Expr => Group::Expr(tokens.clone()),
            Type::Closure => Group::Closure(tokens.clone()),
            Type::List => Group::List(self.group_by_comma(tokens.clone())),
            Type::Object => Group::Object(self.hash_from_tokens(tokens.clone())),
        };
        args.push(group);
        *tokens = Vec::new();
        *typ = Type::Expr;
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
                key = {
                    let mut tokens = Tokens::new();
                    tree.to_tokens(&mut tokens);
                    tokens.to_string()
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

        for (i, tree) in tt.into_iter().enumerate() {
            let token_is_comma = if let TokenTree::Token(Token::Comma) = tree { true } else { false };
            let on_last_token = if i == last { true } else { false };
            let arg_end = token_is_comma || on_last_token;

            match Type::new(&tokens, &tree, &mut typ) {
                Type::Expr => {
                    if !token_is_comma {
                        tokens.push(tree);
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ);
                    }
                }
                Type::Closure => {
                    if !token_is_comma {
                        tokens.push(tree);
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ);
                    }
                }
                Type::List => {
                    if !token_is_comma {
                        if let TokenTree::Delimited(d) = tree {
                            tokens = d.tts;
                        }
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ);
                    }
                }
                Type::Object => {
                    if !token_is_comma {
                        if let TokenTree::Delimited(d) = tree {
                            tokens = d.tts;
                        }
                    }
                    if arg_end {
                        self.finalise_arg(&mut args, &mut tokens, &mut typ);
                    }
                }
            }
        }
        args
    }
}

impl Type {
    fn new(tokens: &Vec<TokenTree>, tree: &TokenTree, typ: &mut Type) -> Type {
        if tokens.is_empty() {
            if let TokenTree::Delimited(ref d) = *tree {
                if d.delim == DelimToken::Bracket { *typ = Type::List; }
                else if d.delim == DelimToken::Brace { *typ = Type::Object; }
            }
            else if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = *tree {
                *typ = Type::Closure;
            }
        } else if tokens.len() == 1 {
            if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = *tree {
                if let TokenTree::Token(Token::Ident(ref i)) = tokens[0] {
                    if i == "move" {
                        *typ = Type::Closure;
                    }
                }
            }
        }
        *typ
    }
}
