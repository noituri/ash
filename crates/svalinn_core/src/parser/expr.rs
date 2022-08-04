use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::{common::ident_parser, function::call_parse};

#[derive(Debug)]
pub(crate) enum Expr {
    Variable(String),
    Call { callee: Box<Expr>, args: Vec<Expr> },
}

pub(super) type ExprRecursive<'a> = Recursive<'a, Token, Expr, Simple<Token>>;

pub(super) fn expression_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    // let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);
    recursive(|expr| {
        let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);
        call_parse(expr).or(variable)
        // variable.or(call_parse(expr))
    })

    // variable
}
