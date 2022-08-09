use crate::lexer::token::Token;
use chumsky::prelude::*;

use super::{
    expr::{expression_parser, Expr},
    function::function_parser,
};

#[derive(Debug)]
pub(crate) enum Stmt {
    Function {
        name: String,
        args: Vec<(String, String)>,
        body: Vec<Stmt>,
        ty: String, // TODO: use Ty enum,
    },
    Expression(Expr),
}

pub(super) type StmtRecursive<'a> = Recursive<'a, Token, Stmt, Simple<Token>>;

pub(super) fn statement_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Stmt, Error = Simple<Token>> + 'a {
    // recursive(|stmt| function_parser(stmt).or(expression_parser().map(Stmt::Expression)))
    //     .debug("STMT")
    let expr = expression_parser()
        .then_ignore(just(Token::NewLine))
        .debug("EXPR STMT")
        .map(Stmt::Expression);
    function_parser(stmt).or(expr)
    // .padded_by(just(Token::NewLine).repeated())
}
