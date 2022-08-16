use crate::{
    core::{annotation::Annotation, Spanned},
    lexer::token::Token,
};
use chumsky::prelude::*;

use super::{common::ident_parser, function::{function_parser, function_proto_parser}, Stmt, StmtRecursive};

pub(super) fn annotation_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::At)
        .ignore_then(ident_parser().delimited_by(just(Token::LBracket), just(Token::RBracket)))
        .then_ignore(just(Token::NewLine).or_not())
        .map_with_span(|name, span| (name, span))
        .then(function_parser(stmt).or(function_proto_parser())) // Support only for functions for now
        .map_with_span(|((name, name_span), stmt), span| {
            (
                Stmt::Annotation((Annotation::new(name), name_span), Box::new(stmt)),
                span,
            )
        })
}
