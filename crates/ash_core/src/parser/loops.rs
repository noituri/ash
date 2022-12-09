use crate::{core::Spanned, lexer::token::Token};

use super::{expression_parser, Stmt, StmtRecursive, common::stmt_block_parser};
use chumsky::prelude::*;

pub(super) fn while_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::While)
        .ignore_then(expression_parser().map_with_span(|e, s| (e, s)))
        .then(stmt_block_parser(stmt))
        .map_with_span(|(cond, body), span| (Stmt::While(cond, body.0.block_data()), span))
}
