use crate::{lexer::token::Token, core::Spanned};

use super::{common::block_parser, expression_parser, Stmt, StmtRecursive};
use chumsky::prelude::*;

pub(super) fn while_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::While)
        .ignore_then(expression_parser().map_with_span(|e, s| (e, s)))
        .then(block_parser(stmt))
        .map_with_span(|(cond, body), span| (Stmt::While(cond, body.block_data()), span))
}
