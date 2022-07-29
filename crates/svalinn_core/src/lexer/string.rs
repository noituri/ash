use crate::lexer::token::{Token, TokenTree};
use chumsky::prelude::*;

pub(super) fn string_lexer() -> impl Parser<char, TokenTree, Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|s| Token::String(s).to_tree())
}
