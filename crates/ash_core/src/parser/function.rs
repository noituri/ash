use crate::core::Spanned;
use crate::parser::stmt::Stmt;
use crate::ty::function::{Function, ProtoFunction};
use crate::ty::Ty;
use crate::{core::next_id, lexer::token::Token};
use chumsky::prelude::*;

use super::common::{type_parser, stmt_block_parser};
use super::expression_parser;
use super::{
    common::{ident_parser, ident_with_suffix_parser},
    expr::{Expr, ExprRecursive},
    stmt::{stmt_expression_parser, StmtRecursive},
};

pub(super) fn function_proto_parser() -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> {
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
    let return_type = just(Token::Gt)
        .ignore_then(type_parser())
        .labelled("function return type");

    just(Token::Function)
        .ignore_then(name)
        .then(params)
        .then(return_type.or_not())
        .map_with_span(|((name, params), ty), span| {
            let ty = {
                let param_types = params.iter().map(|p| p.1.clone()).collect::<Vec<_>>();
                let ty = ty.unwrap_or_default();

                Ty::Fun(param_types, Box::new(ty))
            };
            let params = params
                .into_iter()
                .map(|(name, ty)| (next_id(), name, ty))
                .collect::<Vec<_>>();
            let proto = ProtoFunction {
                id: next_id(),
                name,
                params,
                ty,
            };

            (Stmt::ProtoFunction(proto), span)
        })
        .labelled("function")
}

pub(super) fn function_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    let body = just(Token::Arrow)
        .ignore_then(expression_parser())
        .then_ignore(just(Token::SemiColon))
        .map_with_span(|expr, span| (Stmt::Expression(expr), span))
        .or(stmt_block_parser(stmt))
        .map_with_span(|stmt, span| (stmt.0, span));

    function_proto_parser()
        .then(body)
        .map_with_span(|(proto, body), span| {
            let fun = Function {
                body,
                proto: (proto.0.proto_fun(), proto.1),
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
    callee.then(args).foldl(|callee, args| Expr::Call {
        args,
        callee: Box::new(callee),
    })
}

pub(super) fn return_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::Ret)
        .ignore_then(
            stmt_expression_parser(stmt)
                .or_not()
                .then_ignore(just(Token::SemiColon))
        )
        .map_with_span(|expr, span| (Stmt::Return(expr), span))
}
