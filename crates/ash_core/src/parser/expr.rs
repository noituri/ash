use crate::{
    core::{next_id, Id, Spanned},
    lexer::token::Token,
    ty::Value,
};
use chumsky::prelude::*;

use super::{
    common::ident_parser,
    function::call_parser,
    literal::literal_parser,
    operator::{operator_parser, BinaryOp, UnaryOp},
    stmt::Stmt,
};

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Variable(Id, String),
    Literal(Value),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Block(Vec<Spanned<Stmt>>),
    Group(Box<Expr>),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

pub(super) type ExprRecursive<'a> = Recursive<'a, Token, Expr, Simple<Token>>;

pub(super) fn expression_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let variable = ident_parser()
            .debug("VARIABLE EXPR")
            .map(|name| Expr::Variable(next_id(), name));
        let group = expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(|e| Expr::Group(Box::new(e)));

        let expr = literal_parser()
            .or(call_parser(expr.clone()))
            .or(variable)
            .or(group);
        // .then(operator_parser(expr.clone()))
        operator_parser(expr)
    })
}
