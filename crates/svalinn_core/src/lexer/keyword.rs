use chumsky::prelude::*;

use crate::lexer::token::Token;

pub(super) fn keyword_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident().map(|ident: String| match ident.as_str() {
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    })
}
