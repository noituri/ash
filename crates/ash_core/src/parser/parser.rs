use crate::{core::Spanned, lexer::token::Token, prelude::AshResult};
use chumsky::{prelude::*, Parser as ChumskyParser, Stream};

use super::{stmt::{statement_parser, Stmt}};

pub(crate) struct Parser<'a>(BoxedParser<'a, Token, Vec<Spanned<Stmt>>, Simple<Token>>);

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let parser = statement_parser().repeated();
        Self(parser.then_ignore(end()).boxed())
    }

    // TODO: Return spanned Stmt
    pub fn parse(&self, tokens: Vec<Spanned<Token>>) -> AshResult<Vec<Spanned<Stmt>>, Token> {
        let len = tokens.len();
        let tokens = Stream::from_iter(len..len + 1, tokens.into_iter());
        self.0.parse(tokens)
    }
}
