use chumsky::{prelude::*, text::Character};

use crate::lexer::token::Token;

pub(super) fn keyword_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident()
        .then(filter(|c: &char| c.is_inline_whitespace()).repeated())
        .map(|(ident, space): (String, Vec<_>)| match ident.as_str() {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "return" => Token::Return,
            "fun" => Token::Function,
            _ => Token::Identifier {
                value: ident,
                space_sufix: !space.is_empty(),
            },
        })
}
