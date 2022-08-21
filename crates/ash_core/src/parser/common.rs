use crate::{lexer::token::Token, ty::Ty};
use chumsky::prelude::*;

use super::{expr::Expr, stmt::StmtRecursive};

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

pub(super) fn block_parser<'a>(
    stmt: StmtRecursive<'a>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a + Clone {
    just(Token::LBrace)
        .ignore_then(stmt.repeated())
        .padded_by(just(Token::NewLine).repeated())
        .then_ignore(just(Token::RBrace))
        .map(Expr::Block)
        // .recover_with(skip_until([Token::LBrace, Token::RBrace], |_| Expr::Block(Vec::new())))
        // .recover_with(nested_delimiters(Token::LBrace, Token::RBrace, [], |_| Expr::Block(Vec::new())))
}

pub(super) fn type_parser() -> impl Parser<Token, Ty, Error = Simple<Token>> {
    ident_parser().map::<Ty, _>(From::from)
}
