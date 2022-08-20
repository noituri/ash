use crate::{core::Spanned, lexer::token::Token};
use chumsky::prelude::*;

use super::{common::block_parser, expression_parser, Expr, StmtRecursive, Stmt};

#[derive(Debug, Clone)]
pub(crate) struct If<E, S> {
    pub then: Box<IfInner<E, S>>,
    pub else_ifs: Vec<IfInner<E, S>>,
    pub otherwise: Vec<Spanned<S>>,
}

#[derive(Debug, Clone)]
pub(crate) struct IfInner<E, S> {
    pub condition: Spanned<E>,
    pub body: Vec<Spanned<S>>,
}

pub(super) fn if_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    let then = just(Token::If)
        .ignore_then(expression_parser())
        .map_with_span(|cond, span| (cond, span))
        .then(block_parser(stmt.clone()))
        .map(|(condition, block)| IfInner {
            condition,
            body: block.block_data(),
        });

    let else_if = just(Token::Else).ignore_then(then.clone());

    let otherwise = just(Token::Else)
        .ignore_then(block_parser(stmt))
        .map(|block| block.block_data());

    then.labelled("if")
        .then(else_if.labelled("else if").repeated())
        .then(otherwise.labelled("else").or_not())
        .map(|((then, else_ifs), otherwise)| Expr::If(If {
            then: Box::new(then),
            else_ifs,
            otherwise: otherwise.unwrap_or(Vec::new()),
        }))
}
