use chumsky::prelude::*;

use crate::lexer::token::Token;

use super::{
    common::ident_parser,
    stmt::{stmt_expression_parser, Stmt, StmtRecursive},
};

pub(super) fn variable_decl_parse<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Stmt, Error = Simple<Token>> + 'a {
    // TODO: Use (ty) type parsing for variable type
    just(Token::Let)
        .ignore_then(ident_parser())
        .then(just(Token::Colon).ignore_then(ident_parser()).or_not())
        .then_ignore(just(Token::Equal))
        .then(stmt_expression_parser(stmt).then_ignore(just(Token::NewLine)))
        .map(|((name, ty), value)| Stmt::VariableDecl { name, ty, value })
}

pub(super) fn variable_assign_parse<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Stmt, Error = Simple<Token>> + 'a {
    ident_parser()
        .then_ignore(just(Token::Equal))
        .then(stmt_expression_parser(stmt).then_ignore(just(Token::NewLine)))
        .map(|(name, value)| Stmt::VariableAssign { name, value })
}
