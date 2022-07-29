use chumsky::prelude::*;

use crate::lexer::token::{Token, TokenTree};

pub(super) fn keyword_lexer() -> impl Parser<char, TokenTree, Error = Simple<char>> {
    text::ident().map(|ident: String| {
        match ident.as_str() {
            "fn" => Token::Fn,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Identifier(ident),
        }
        .to_tree()
    })
}
