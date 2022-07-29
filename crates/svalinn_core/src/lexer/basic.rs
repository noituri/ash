use super::token::Token;
use crate::lexer::token::TokenTree;
use chumsky::prelude::*;

pub(super) fn basic_lexer() -> impl Parser<char, TokenTree, Error = Simple<char>> {
    let arrow = just("->").to(Token::Arrow.to_tree());

    let simple = one_of("+-=*/").map(|c| {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '=' => Token::Equal,
            ',' => Token::Comma,
            _ => unreachable!(),
        }
        .to_tree()
    });

    arrow.or(simple)
}
