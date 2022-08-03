use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::stmt::{statement_parser, Stmt, StmtRecursive};

pub(super) fn ident_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    filter_map(|span, tok| match tok {
        Token::Identifier(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(
            span,
            vec![Some(Token::Identifier("".to_owned()))],
            Some(tok),
        )),
    })
}

pub(super) fn block_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Vec<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::StartBlock)
        .ignore_then(stmt.repeated())
        .then_ignore(just(Token::EndBlock))
        .debug("BLOCK NEW")
    // stmt.repeated()
    //     .debug("BLOCK")
    //     .delimited_by(just(Token::StartBlock), just(Token::EndBlock))
}
