use crate::{core::Spanned, lexer::token::Token, prelude::Span, ty::Ty};
use chumsky::prelude::*;

use super::{expr::Expr, stmt::StmtRecursive, Stmt, stmt_expression_parser};

pub(super) fn ident_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    filter_map(|span, tok| match tok {
        Token::Identifier { value, .. } => Ok(value.clone()),
        _ => Err(Simple::expected_input_found(
            span,
            vec![Some(Token::Identifier {
                value: "".to_owned(),
                space_sufix: false,
            })],
            Some(tok),
        )),
    })
}

pub(super) fn ident_with_suffix_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone
{
    filter_map(|span, tok| match tok {
        Token::Identifier {
            value,
            space_sufix: true,
        } => Ok(value.clone()),
        _ => Err(Simple::expected_input_found(
            span,
            vec![Some(Token::Identifier {
                value: "".to_owned(),
                space_sufix: false,
            })],
            Some(tok),
        )),
    })
}

pub(super) fn expr_block_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a + Clone {
    just(Token::LBrace)
        .ignore_then(stmt.repeated())
        .then_ignore(just(Token::RBrace))
        .map(Expr::Block)
    // .recover_with(skip_until([Token::LBrace, Token::RBrace], |_| Expr::Block(Vec::new())))
    // .recover_with(nested_delimiters(Token::LBrace, Token::RBrace, [], |_| Expr::Block(Vec::new())))
}

pub(super) fn stmt_block_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a + Clone {
    just(Token::LBrace)
        .ignore_then(stmt.repeated())
        .then_ignore(just(Token::RBrace))
        .map_with_span(|stmts, span| (Stmt::Block(stmts), span))
    // .recover_with(skip_until([Token::LBrace, Token::RBrace], |_| Expr::Block(Vec::new())))
    // .recover_with(nested_delimiters(Token::LBrace, Token::RBrace, [], |_| Expr::Block(Vec::new())))
}

pub(super) fn type_parser() -> impl Parser<Token, Ty, Error = Simple<Token>> {
    ident_parser().map::<Ty, _>(From::from)
}

pub(crate) fn calc_block_span<T>(statements: &[Spanned<T>], start_span: Span) -> Span {
    let first = match statements.first() {
        Some((_, span)) => span,
        None => return start_span,
    };

    let last = &statements.last().unwrap().1;

    first.start()..last.end()
}


pub(super) fn break_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Spanned<Stmt>, Error = Simple<Token>> + 'a {
    just(Token::Break)
        .ignore_then(
            stmt_expression_parser(stmt)
                .or_not()
                .then_ignore(just(Token::SemiColon))
        )
        .map_with_span(|expr, span| (Stmt::Break(expr), span))
}