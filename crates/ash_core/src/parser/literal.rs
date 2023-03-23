use chumsky::prelude::*;

use crate::{lexer::token::Token, ty::Value};

use super::expr::Expr;

pub(super) fn literal_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone {
    select! {
        Token::I32(value) => Expr::Literal(Value::I32(value.parse().unwrap())),
        Token::F64(value) => Expr::Literal(Value::F64(value.parse().unwrap())),
        Token::Str(value) => Expr::Literal(Value::String(value)),
        Token::Bool(value) => Expr::Literal(Value::Bool(value)),
    }
    .labelled("literal")
}
