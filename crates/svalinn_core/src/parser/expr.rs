use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::common::ident_parser;

#[derive(Debug)]
pub(crate) enum Expr {
    Variable(String),
}

pub(super) fn expression_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    let variable = ident_parser().debug("VARIABLE EXPR").map(Expr::Variable);

    variable
}
