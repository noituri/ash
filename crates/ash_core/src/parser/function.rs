use crate::common::Spanned;
use crate::parser::stmt::Stmt;
use crate::ty::function::Function;
use crate::ty::Ty;
use crate::{common::next_id, lexer::token::Token};
use chumsky::prelude::*;

use super::common::type_parser;
use super::{
    common::{ident_parser, ident_with_suffix_parser},
    expr::{Expr, ExprRecursive},
    stmt::{stmt_expression_parser, StmtRecursive},
};

pub(super) fn function_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    let ident = ident_parser();

    let name = ident.clone().labelled("function name");
    let params = ident
        .clone()
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .labelled("function args");
    let return_type = just(Token::Colon)
        .ignore_then(type_parser())
        .labelled("function return type");
    let body = just(Token::Equal)
        .ignore_then(stmt_expression_parser(stmt))
        .then_ignore(just(Token::NewLine))
        .map_with_span(|expr, span| (Stmt::Expression(expr), span));

    just(Token::Function)
        .ignore_then(name)
        .then(params.or_not())
        .then(return_type.or_not())
        .then(body)
        .map_with_span(|(((name, params), ty), body), span| {
            let params = params.unwrap_or_default();
            let ty = {
                let param_types = params.iter().map(|p| p.1.clone()).collect::<Vec<_>>();
                let ty = ty.unwrap_or_default();

                Ty::Fun(param_types, Box::new(ty))
            };
            let params = params
                .into_iter()
                .map(|(name, ty)| (next_id(), name, ty))
                .collect::<Vec<_>>();

            let fun = Function {
                id: next_id(),
                name,
                params,
                ty,
                body,
            };

            (Stmt::Function(Box::new(fun)), span)
        })
        .labelled("function")
}

pub(super) fn call_parser<'a>(
    expr: ExprRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    let callee = ident_parser().map(|name| Expr::Variable(next_id(), name));
    let args = expr
        .clone()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .repeated();
    let call = callee.then(args).foldl(|callee, args| Expr::Call {
        args,
        callee: Box::new(callee),
    });

    call_no_parens_parser(expr).or(call)
}

pub(super) fn call_no_parens_parser<'a>(
    expr: ExprRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a {
    let callee = ident_with_suffix_parser().map(|name| Expr::Variable(next_id(), name));
    let args = expr.clone().separated_by(just(Token::Comma)).at_least(1);
    callee.then(args).map(|(callee, args)| Expr::Call {
        args,
        callee: Box::new(callee),
    })
}

pub(super) fn return_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::Return)
        .ignore_then(stmt_expression_parser(stmt).then_ignore(just(Token::NewLine)))
        .map_with_span(|expr, span| (Stmt::Return(expr), span))
}
