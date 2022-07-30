use chumsky::prelude::*;

use crate::lexer::token::{Token};

use super::token::TokenType;

pub(super) fn basic_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    let arrow = just("->").to(TokenType::Arrow.to_token());

    let ops = one_of("+-*/")
        .map_with_span(|c, _span| {
            match c {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '/' => TokenType::Slash,
                '*' => TokenType::Asterisk,
                _ => unreachable!(),
            }
            .to_token()
        })
        .labelled("operators");

    let other = one_of("=,").map_with_span(|c, _span| {
        match c {
            '=' => TokenType::Equal,
            ',' => TokenType::Comma,
            _ => unreachable!(),
        }
        .to_token()
    });

    arrow.or(ops).or(other)
}
