use crate::{lexer::token::Token, common::{Id, Spanned}, ty::{function::Function, Ty}};
use chumsky::prelude::*;

use super::{
    common::block_parser,
    expr::{expression_parser, Expr},
    function::{function_parser, return_parser},
    variable::{variable_assign_parse, variable_decl_parse},
};

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Function(Box<Function<Stmt>>),
    VariableDecl {
        id: Id,
        name: String,
        ty: Option<Ty>,
        value: Expr,
    },
    VariableAssign {
        id: Id,
        name: Spanned<String>,
        value: Expr,
    },
    Return(Expr),
    Expression(Expr),
}

pub(super) type StmtRecursive<'a> = Recursive<'a, Token, Spanned<Stmt>, Simple<Token>>;

pub(super) fn statement_parser() -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> {
    recursive(|stmt| {
        let expr = expression_parser()
            .then_ignore(just(Token::NewLine))
            .map_with_span(|expr, span| (Stmt::Expression(expr), span));

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
