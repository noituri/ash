use crate::{
    core::{annotation::Annotation, Id, Spanned},
    lexer::token::Token,
    ty::{
        function::{Function, ProtoFunction},
        Ty,
    },
};
use chumsky::prelude::*;

use super::{
    annotation::annotation_parser,
    common::block_parser,
    conditional::if_parser,
    expr::{expression_parser, Expr},
    function::{function_parser, function_proto_parser, return_parser},
    variable::{variable_assign_parse, variable_decl_parse},
};

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Annotation(Spanned<Annotation>, Box<Spanned<Stmt>>),
    ProtoFunction(ProtoFunction),
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
    Return(Option<Expr>),
    Expression(Expr),
}

impl Stmt {
    pub fn proto_fun(self) -> ProtoFunction {
        match self {
            Self::ProtoFunction(proto) => proto,
            _ => panic!("Not proto function"),
        }
    }
}

pub(super) type StmtRecursive<'a> = Recursive<'a, Token, Spanned<Stmt>, Simple<Token>>;

pub(super) fn statement_parser() -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> {
    recursive(|stmt| {
        let expr = stmt_expression_parser(stmt.clone())
            .then_ignore(just(Token::NewLine))
            .map_with_span(|expr, span| (Stmt::Expression(expr), span));

        annotation_parser(stmt.clone())
            .or(function_parser(stmt.clone()))
            .or(function_proto_parser())
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
    if_parser(stmt.clone())
        .or(block_parser(stmt))
        .or(expression_parser())
}
