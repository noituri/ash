use chumsky::prelude::*;

use crate::{
    core::{next_id, Spanned},
    lexer::token::Token,
};

use super::{
    common::{ident_parser, type_parser},
    stmt::{stmt_expression_parser, Stmt, StmtRecursive},
};

pub(super) fn variable_decl_parse<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::Val)
        .or(just(Token::Var))
        .then(ident_parser())
        .then(just(Token::Colon).ignore_then(type_parser()).or_not())
        .then_ignore(just(Token::Equal))
        .then(stmt_expression_parser(stmt).then_ignore(just(Token::SemiColon)))
        .map_with_span(|(((tok, name), ty), value), span| {
            (
                Stmt::VariableDecl {
                    id: next_id(),
                    name,
                    ty,
                    value,
                    mutable: tok == Token::Var,
                },
                span,
            )
        })
}

pub(super) fn variable_assign_parse<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    ident_parser()
        .map_with_span(|name, span| (name, span))
        .then_ignore(just(Token::Equal))
        .then(stmt_expression_parser(stmt).then_ignore(just(Token::SemiColon)))
        .map_with_span(|(name, value), span| {
            (
                Stmt::VariableAssign {
                    id: next_id(),
                    name,
                    value,
                },
                span,
            )
        })
}
