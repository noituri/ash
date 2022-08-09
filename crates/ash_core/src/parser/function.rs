use crate::lexer::token::Token;
use crate::parser::stmt::Stmt;
use chumsky::prelude::*;

use super::{
    common::{block_parser, ident_parser, ident_with_suffix_parser},
    expr::{expression_parser, Expr, ExprRecursive},
    stmt::{stmt_expression_parser, StmtRecursive},
};

pub(super) fn function_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Stmt, Error = Simple<Token>> + 'a {
    let ident = ident_parser();

    let name = ident.clone().labelled("function name");
    let args = ident
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
        .map(|expr| vec![Stmt::Expression(expr)]);

    let decl = just(Token::Function)
        .ignore_then(name)
        .then(args.or_not())
        .then(return_type.or_not())
        .then(body)
        .labelled("function");

    decl.map(|(((name, args), ty), body)| Stmt::Function {
        name,
        body,
        args: args.unwrap_or_default(),
        ty: ty.unwrap_or("Void".to_owned()),
    })
}

pub(super) fn call_parser<'a>(
    expr: ExprRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    let callee = ident_parser().map(Expr::Variable);
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
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    // TODO: convert variable of Function type to a callee
    let callee = ident_with_suffix_parser().map(Expr::Variable);
    let args = expr.clone().separated_by(just(Token::Comma)).at_least(1);
    callee.then(args).map(|(callee, args)| Expr::Call {
        args,
        callee: Box::new(callee),
    })
}
