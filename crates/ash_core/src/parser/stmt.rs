use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::{
    common::block_parser,
    expr::{expression_parser, Expr},
    function::{function_parser, return_parser}, variable::{variable_decl_parse, variable_assign_parse},
};

#[derive(Debug)]
pub(crate) enum Stmt {
    Function {
        name: String,
        args: Vec<(String, String)>,
        body: Box<Stmt>,
        ty: String, // TODO: use Ty enum,
    },
    VariableDecl {
        name: String,
        ty: Option<String>, // TODO: use Ty enum
        value: Expr,
    },
    VariableAssign {
        name: String,
        value: Expr,
    },
    Return(Expr),
    Expression(Expr),
}

pub(super) type StmtRecursive<'a> = Recursive<'a, Token, Stmt, Simple<Token>>;

pub(super) fn statement_parser() -> impl Parser<Token, Stmt, Error = Simple<Token>> {
    recursive(|stmt| {
        let expr = expression_parser()
            .then_ignore(just(Token::NewLine))
            .map(Stmt::Expression);

        function_parser(stmt.clone())
            .or(variable_decl_parse(stmt.clone()))
            .or(variable_assign_parse(stmt.clone()))
            .or(return_parser(stmt))
            .or(expr)
            .padded_by(just(Token::NewLine).repeated())
    })
}

pub(super) fn stmt_expression_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    block_parser(stmt).or(expression_parser())
}
