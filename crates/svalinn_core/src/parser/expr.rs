use crate::lexer::token::Token;
use crate::parser::operator::unary_parser;
use chumsky::prelude::*;

use super::{common::ident_parser, function::call_parser, operator::UnaryOp};

#[derive(Debug)]
pub(crate) enum Expr {
    Variable(String),
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Group(Box<Expr>),
    Unary { op: UnaryOp, right: Box<Expr> },
}

pub(super) type ExprRecursive<'a> = Recursive<'a, Token, Expr, Simple<Token>>;

pub(super) fn expression_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);
        let group = expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(|e| Expr::Group(Box::new(e)));

        unary_parser(expr.clone())
            .or(call_parser(expr.clone()))
            .or(variable)
            .or(group)
    })
}
