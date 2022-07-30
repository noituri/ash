use chumsky::prelude::*;

use crate::lexer::token::{Token, TokenTree, TokenType};
use crate::ty::Value;

pub(super) fn string_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::string)
        .labelled("String")
}
