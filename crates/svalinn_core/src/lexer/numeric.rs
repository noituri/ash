use std::io::Read;

use crate::common::Span;
use chumsky::prelude::*;

use crate::lexer::token::{Token, TokenTree, TokenType};

pub(super) fn numeric_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| Token::integer(s.parse().unwrap()))
        .labelled("I32");

    let double = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)))
        .collect::<String>()
        .map(|s: String| Token::float(s.parse().unwrap()))
        .labelled("F64");

    double.or(int)
}
