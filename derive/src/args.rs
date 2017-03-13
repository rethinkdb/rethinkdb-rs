use syn::{self, Token, TokenTree, BinOpToken, DelimToken};
use quote::{Tokens, ToTokens};

#[derive(Debug)]
pub struct Args<'a> {
    input: &'a str,
    pub tokens: Tokens,
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Object,
    List,
    Closure,
    Expr,
}

#[derive(Debug, Clone)]
struct Group {
    tree: Vec<TokenTree>,
    typ: Type,
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
    fn body(&self) -> Tokens {
        let args = self.group_by_comma();
        let args: Vec<String> = args.into_iter()
            .map(|arg| {
                let mut tokens = Tokens::new();
                for arg in arg.tree {
                    arg.to_tokens(&mut tokens);
                }
                format!("{:?} => {}", arg.typ, tokens)
            })
        .collect();
        panic!(format!("{:?}", args));
    }

    fn group_by_comma(&self) -> Vec<Group> {
        let mut args = Vec::new();
        let tt = syn::parse_token_trees(self.input).expect("failed to parse token tree");
        let mut group = Group::new();
        for tree in tt {
            if group.is_empty() {
                if let TokenTree::Delimited(ref d) = tree {
                    if d.delim == DelimToken::Bracket { group.typ = Type::List; }
                    else if d.delim == DelimToken::Brace { group.typ = Type::Object; }
                }
                else if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = tree {
                    group.typ = Type::Closure;
                }
            } else if group.tree.len() == 1 {
                if let TokenTree::Token(Token::BinOp(BinOpToken::Or)) = tree {
                    if let TokenTree::Token(Token::Ident(ref i)) = group.tree[0] {
                        if i == "move" {
                            group.typ = Type::Closure;
                        }
                    }
                }
            }
            if let TokenTree::Token(Token::Comma) = tree {
                if !group.is_empty() {
                    args.push(group);
                    group = Group::new();
                }
            } else {
                group.tree.push(tree);
            }
        }
        if !group.is_empty() {
            args.push(group);
        }
        args
    }
}

impl Group {
    fn new() -> Self {
        Group {
            tree: Vec::new(),
            typ: Type::Expr,
        }
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}
