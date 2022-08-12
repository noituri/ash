use crate::common::Spanned;
use crate::parser::stmt::Stmt;
use crate::{common::next_id, lexer::token::Token};
use chumsky::prelude::*;

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
        .then(ident.clone())
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .labelled("function args");
    let return_type = just(Token::Colon)
        .ignore_then(ident)
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
            let fun = Stmt::Function {
                name,
                body: Box::new(body),
                params: params.unwrap_or_default(),
                ty: ty.unwrap_or("Void".to_owned()),
            };

            (fun, span)
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
    // TODO: convert variable of Function type to a callee
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
