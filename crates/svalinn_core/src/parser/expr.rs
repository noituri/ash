use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::{common::ident_parser, function::call_parse};

#[derive(Debug)]
pub(crate) enum Expr {
    Variable(String),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        has_parens: bool,
    },
    Group(Box<Expr>),
}

pub(super) type ExprRecursive<'a> = Recursive<'a, Token, Expr, Simple<Token>>;

pub(super) fn expression_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    // let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);
    recursive(|expr| {
        let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);
        let group = expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(|e| Expr::Group(Box::new(e)));
        // let callable = variable.clone();
        // let call_args = expr.
        call_parse(expr.clone()).or(variable).or(group)
        // .or(variable)

        // variable.or(call_parse(expr))
    })
    // .or(inner_expression_parser())

    // variable
}
