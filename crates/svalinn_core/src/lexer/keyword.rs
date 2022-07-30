use crate::common::Span;
use chumsky::prelude::*;

use crate::lexer::token::{Token, TokenTree, TokenType};

pub(super) fn keyword_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident().map(|ident: String| match ident.as_str() {
        "fn" => TokenType::Fn.to_token(),
        "true" => Token::boolean(true),
        "false" => Token::boolean(false),
        _ => TokenType::Identifier(ident).to_token(),
    })
}
