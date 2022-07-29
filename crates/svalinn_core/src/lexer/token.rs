use crate::lexer::Span;

#[derive(Clone, Debug)]
pub(crate) enum Token {
    LParen,
    RParen,
    Equal,
    Minus,
    Plus,
    Asterisk,
    Slash,
    Arrow,
    Comma,
    Identifier(String),
    Fn,
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
}

impl Token {
    pub fn to_tree(self) -> TokenTree {
        TokenTree::Token(self)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Delim {
    Paren,
    Brace,
    Bracket,
    Block,
}

#[derive(Clone, Debug)]
pub(crate) enum TokenTree {
    Token(Token),
    Tree(Delim, Vec<(TokenTree, Span)>),
}
