use crate::lexer::token::{Token, TokenTree};
use chumsky::prelude::*;
use std::io::Read;

pub(super) fn numeric_lexer() -> impl Parser<char, TokenTree, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| Token::I32(s.parse().unwrap()).to_tree())
        .labelled("I32");

    let double = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)))
        .collect::<String>()
        .map(|s: String| Token::F64(s.parse().unwrap()).to_tree())
        .labelled("F64");

    double.or(int)
}
