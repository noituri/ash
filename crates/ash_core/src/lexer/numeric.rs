use chumsky::prelude::*;

use crate::lexer::token::Token;

pub(super) fn numeric_lexer() -> impl Parser<char, Token, Error = Simple<char>> {
    let int = text::int(10).map(Token::I32).labelled("i32");

    let double = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)))
        .collect::<String>()
        .map(Token::F64)
        .labelled("f64");

    double.or(int)
}
