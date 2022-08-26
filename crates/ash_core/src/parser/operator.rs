use chumsky::prelude::*;

use crate::lexer::token::Token;

use super::expr::Expr;

#[derive(Debug, Clone)]
pub(crate) enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
    Rem,
    Equal,
    NotEqual,
    Gt,
    Lt,
    Gte,
    Lte,
}

pub(super) fn operator_parser<'a, P>(
    expr: P,
) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a
where
    P: Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
{
    binary_parser(unary_parser(expr))
}

fn unary_parser<'a, P>(expr: P) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a
where
    P: Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
{
    let minus = just(Token::Minus)
        .repeated()
        .then(expr.clone())
        .foldr(|_, rhs| Expr::Unary {
            op: UnaryOp::Neg,
            right: Box::new(rhs),
        });
    let not = just(Token::Bang)
        .repeated()
        .then(expr)
        .foldr(|_, rhs| Expr::Unary { op: UnaryOp::Not, right: Box::new(rhs) });

    minus.or(not)
}

fn binary_parser<'a, P>(expr: P) -> impl Parser<Token, Expr, Error = Simple<Token>> + 'a
where
    P: Parser<Token, Expr, Error = Simple<Token>> + Clone + 'a,
{
    let op = just(Token::Asterisk)
        .to(BinaryOp::Mul)
        .or(just(Token::Slash).to(BinaryOp::Div));
    let product = expr
        .clone()
        .then(op.then(expr).repeated())
        .foldl(|a, (op, b)| Expr::Binary {
            left: Box::new(a),
            op,
            right: Box::new(b),
        });

    let op = just(Token::Plus)
        .to(BinaryOp::Sum)
        .or(just(Token::Minus).to(BinaryOp::Sub));
    let sum = product
        .clone()
        .then(op.then(product).repeated())
        .foldl(|a, (op, b)| Expr::Binary {
            left: Box::new(a),
            op,
            right: Box::new(b),
        });

    let op = just(Token::Gt)
        .to(BinaryOp::Gt)
        .or(just(Token::Lt).to(BinaryOp::Gt))
        .or(just(Token::Gte).to(BinaryOp::Gte))
        .or(just(Token::Lte).to(BinaryOp::Lte));
    let ord = sum
        .clone()
        .then(op.then(sum).repeated())
        .foldl(|a, (op, b)| Expr::Binary { left: Box::new(a), op, right: Box::new(b) });

    let op = just(Token::DoubleEqual)
        .to(BinaryOp::Equal)
        .or(just(Token::NotEqual).to(BinaryOp::NotEqual));
    let equality = ord
        .clone()
        .then(op.then(ord).repeated())
        .foldl(|a, (op, b)| Expr::Binary {
            left: Box::new(a),
            op,
            right: Box::new(b),
        });

    equality
}
