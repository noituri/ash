use crate::lexer::token::Token;
use crate::parser::stmt::Stmt;
use chumsky::prelude::*;

use super::{
    common::{block_parser, ident_parser},
    expr::{expression_parser, Expr, ExprRecursive},
    stmt::StmtRecursive,
};

pub(super) fn function_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Stmt, Error = Simple<Token>> + 'a {
    let ident = ident_parser();

    let name = ident.clone().labelled("function name");
    let args = ident
        .clone()
        .then(ident.clone())
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .labelled("function args");
    let return_type = just(Token::DoubleColon)
        .ignore_then(ident)
        .labelled("function return type");
    let body = just(Token::NewLine)
        .ignore_then(block_parser(stmt).debug("BODY"))
        .or(expression_parser()
            .then_ignore(just(Token::NewLine))
            .map(|expr| vec![Stmt::Expression(expr)]));

    let decl = name
        .then(args.or_not())
        .then(return_type.or_not())
        .then_ignore(just(Token::Arrow))
        .then(body)
        .labelled("function");

    decl.map(|(((name, args), ty), body)| Stmt::Function {
        name,
        body,
        args: args.unwrap_or_default(),
        ty: ty.unwrap_or("Void".to_owned()),
    })
}

pub(super) fn call_parse<'a>(
    expr: ExprRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    let callee = ident_parser().map(Expr::Variable);
    let args = expr
        .clone()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .repeated();
    callee.then(args).foldl(|callee, args| Expr::Call {
        args,
        callee: Box::new(callee),
        has_parens: true,
    })
}
