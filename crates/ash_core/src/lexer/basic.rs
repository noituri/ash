use chumsky::prelude::*;

use super::token::Token;

pub(super) fn basic_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    let arrow = just("=>").to(Token::Arrow);
    let colon = just("::")
        .to(Token::DoubleColon)
        .or(just(':').to(Token::Colon));

    let equal_equal = just("==").to(Token::DoubleEqual);
    let not_equal = just("!=").to(Token::NotEqual);
    let ops = one_of("+-*/%")
        .map_with_span(|c, _span| match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '%' => Token::Percent,
            _ => unreachable!(),
        })
        .labelled("operators");

    let other = one_of("=,").map_with_span(|c, _span| match c {
        '=' => Token::Equal,
        ',' => Token::Comma,
        _ => unreachable!(),
    });

    arrow
        .or(colon)
        .or(equal_equal)
        .or(not_equal)
        .or(ops)
        .or(other)
}
