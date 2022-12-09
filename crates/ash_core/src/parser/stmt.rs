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
    common::{break_parser, stmt_block_parser, expr_block_parser},
    expr::{expression_parser, Expr},
    function::{function_parser, function_proto_parser, return_parser},
    loops::while_parser,
    variable::{variable_assign_parse, variable_decl_parse}, If, stmt_if_parser, expr_if_parser,
};

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Annotation(Spanned<Annotation>, Box<Spanned<Stmt>>),
    ProtoFunction(ProtoFunction),
    Function(Box<Function<Stmt>>),
    If(If<Expr, Stmt>),
    While(Spanned<Expr>, Vec<Spanned<Stmt>>),
    VariableDecl {
        id: Id,
        name: String,
        ty: Option<Ty>,
        value: Expr,
        mutable: bool,
    },
    VariableAssign {
        id: Id,
        name: Spanned<String>,
        value: Expr,
    },
    Block(Vec<Spanned<Stmt>>),
    Break(Option<Expr>),
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

    pub fn block_data(self) -> Vec<Spanned<Stmt>> {
        match self {
            Self::Block(data) => data,
            _ => unreachable!("Not block statement"),
        }
    }
}

pub(super) type StmtRecursive<'a> = Recursive<'a, Token, Spanned<Stmt>, Simple<Token>>;

pub(super) fn statement_parser() -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> {
    recursive(|stmt| {
        let expr = stmt_expression_parser(stmt.clone())
            .then_ignore(just(Token::SemiColon))
            .map_with_span(|expr, span| (Stmt::Expression(expr), span));

        annotation_parser(stmt.clone())
            .or(function_parser(stmt.clone()))
            .or(function_proto_parser())
            .or(while_parser(stmt.clone()))
            .or(variable_decl_parse(stmt.clone()))
            .or(variable_assign_parse(stmt.clone()))
            .or(return_parser(stmt.clone()))
            .or(break_parser(stmt.clone()))
            .or(stmt_block_parser(stmt.clone()))
            .or(stmt_if_parser(stmt))
            .or(expr)
    })
}

pub(super) fn stmt_expression_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a {
    expr_if_parser(stmt.clone())
        .or(expr_block_parser(stmt))
        .or(expression_parser())
}
