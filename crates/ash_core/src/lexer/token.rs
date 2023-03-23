use core::fmt;

use crate::core::Spanned;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    At,
    AndAnd,
    BarBar,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
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
    SemiColon,
    Ret,
    Break,
    Identifier { value: String, space_sufix: bool },
    Function,
    If,
    Else,
    While,
    Val,
    Var,
    Str(String),
    I32(String),
    F64(String),
    Bool(bool),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tok = match self {
            Token::At => "@",
            Token::AndAnd => "&&",
            Token::BarBar => "||",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LBracket => "[",
            Token::RBracket => "]",
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
            Token::SemiColon => ";",
            Token::Ret => "return",
            Token::Break => "break",
            Token::Identifier { .. } => "IDENTIFIER",
            Token::Function => "fun",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::Val => "val",
            Token::Var => "var",
            Token::Str(_) => "str",
            Token::I32(_) => "i32",
            Token::F64(_) => "f64",
            Token::Bool(_) => "bool",
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
