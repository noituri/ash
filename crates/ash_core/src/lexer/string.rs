use chumsky::prelude::*;

use crate::lexer::token::Token;

pub(super) fn string_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String)
        .labelled("String")
}
