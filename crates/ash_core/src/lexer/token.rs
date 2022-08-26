use core::fmt;

use crate::core::Spanned;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    At,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    NewLine,
    Equal,
    DoubleEqual,
    NotEqual,
    Gt,
    Lt,
    Gte,
    Lte,
    Bang,
    Minus,
    Plus,
    Asterisk,
    Percent,
    Slash,
    Arrow,
    Comma,
    Colon,
    DoubleColon,
    Return,
    Identifier { value: String, space_sufix: bool },
    Function,
    If,
    Else,
    While,
    Let,
    String(String),
    I32(String),
    F64(String),
    Bool(bool),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tok = match self {
            Token::At => "@",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::NewLine => "NEWLINE",
            Token::Equal => "=",
            Token::DoubleEqual => "==",
            Token::NotEqual => "!=",
            Token::Gt => ">",
            Token::Lt => "<",
            Token::Gte => ">=",
            Token::Lte => "<=",
            Token::Bang => "!",
            Token::Minus => "-",
            Token::Plus => "+",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Percent => "%",
            Token::Arrow => "=>",
            Token::Comma => ",",
            Token::Colon => ":",
            Token::DoubleColon => "::",
            Token::Return => "return",
            Token::Identifier { .. } => "IDENTIFIER",
            Token::Function => "fun",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::Let => "let",
            Token::String(_) => "String",
            Token::I32(_) => "I32",
            Token::F64(_) => "F64",
            Token::Bool(_) => "Bool",
        };

        f.write_str(tok)
    }
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
}

#[derive(Clone, Debug)]
pub(crate) enum TokenTree {
    Token(Token),
    Tree(Delim, Vec<Spanned<TokenTree>>),
}
