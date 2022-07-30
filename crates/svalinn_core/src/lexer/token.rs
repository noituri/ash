use crate::common::{Spanned};
use crate::ty::Value;

#[derive(Clone, Debug)]
pub(crate) enum TokenType {
    LParen,
    RParen,
    StartBlock,
    EndBlock,
    Equal,
    Minus,
    Plus,
    Asterisk,
    Slash,
    Arrow,
    Comma,
    Identifier(String),
    Fn,
    String,
    I32,
    F64,
    Bool,
}

impl TokenType {
    pub fn to_token(self) -> Token {
        Token::new(self, None)
    }

    pub fn to_tree(self) -> TokenTree {
        self.to_token().to_tree()
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
    Tree(Delim, Vec<Spanned<TokenTree>>),
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub ty: TokenType,
    pub value: Option<Value>,
}

impl Token {
    pub fn new<V: Into<Option<Value>>>(ty: TokenType, value: V) -> Self {
        Self {
            ty,
            value: value.into(),
        }
    }

    pub fn string(s: String) -> Self {
        Self::new(TokenType::String, Value::String(s))
    }

    pub fn integer(i: i32) -> Self {
        Self::new(TokenType::I32, Value::I32(i))
    }

    pub fn float(f: f64) -> Self {
        Self::new(TokenType::F64, Value::F64(f))
    }

    pub fn boolean(b: bool) -> Self {
        Self::new(TokenType::Bool, Value::Bool(b))
    }

    pub fn to_tree(self) -> TokenTree {
        TokenTree::Token(self)
    }
}
